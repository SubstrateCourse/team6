import React, { useEffect, useState } from 'react';
import { Form, Input, Grid, Dropdown,Button } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';

function Main (props) {
  const { api, keyring } = useSubstrate();
  //const { api } = useSubstrate();
  const { accountPair } = props;

  // The transaction submission status
  const [status, setStatus] = useState('');
  const [digest, setDigest] = useState('');
  const [owner, setOwner] = useState('');
  const [accountSelected, setAccountSelected] = useState('');
  const [blockNumber, setBlockNumber] = useState(0);
  const [proofValue, getProofValue] = useState(0);
  const [remarkInfoResult,setRemarkInfoResult] = useState(0);
  const [showingUserDocs, setShowingUserDocs] = useState(false);
  const [userDocs, setUserDocs] = useState([]);
  const [unsub, setUnsub] = useState(null);

  useEffect(() => {
    let unsubscribe;
    api.query.poeModule.proofs(digest,(result) => {
      setOwner(result[0].toString());
      setBlockNumber(result[1].toNumber());
      getProofValue(result[2].toNumber());
      setRemarkInfoResult(result[3].toString());
    }).then(unsub => {
      unsubscribe = unsub;
    })
      .catch(console.error);

    return () => unsubscribe && unsubscribe();
  }, [digest, api.query.poeModule]);

  // Get the list of accounts we possess the private key for
  const keyringOptions = keyring.getPairs().map(account => ({
    key: account.address,
    value: account.address,
    text: account.meta.name.toUpperCase(),
    icon: 'user'
  }));

  const convertToString = (hex) => {
    if (hex && hex.length) {
      return decodeURIComponent(hex.replace(/\s+/g, '').replace(/[0-9a-f]{2}/g, '%$&')).substr(2);
    }
    return '';
  };

  const queryUserDoc = () => {
    unsub && unsub();
    api.query.poeModule.accountHash2List(accountSelected, (result) => {
      setUserDocs([]);
      if (result && result.length) {
        const docs = [];
        result.forEach((digest) => api.query.poeModule.proofs(digest.toString(), (res) => {
          docs.push({
            digest: digest.toString(),
            blockNumber: res[1].toNumber(),
            note: convertToString(res[3].toString())
          });
          if (docs.length === result.length) {
            setUserDocs(docs);
            setShowingUserDocs(true);
            setTimeout(() => setShowingUserDocs(false), 10000);
          }
        }));
      } else {
        setShowingUserDocs(true);
        setTimeout(() => setShowingUserDocs(false), 10000);
      }
    }).then(unsub => setUnsub(unsub))
      .catch(console.error);
  };

  const UserDocs = (props) => {
    const userDocsStyle = {
      marginTop: 10,
      border: '1px solid green',
      backgroundColor: 'lightgreen',
      color: 'darkgreen',
      borderRadius: 5,
      padding: 10
    };
    const { docs } = props;
    if (docs && docs.length) {
      return (
        <div style={ userDocsStyle }>
          <ol>
            {docs.map((doc, index) => <li key={index}>{doc.digest} =&gt; ({doc.blockNumber}, {doc.note})</li>)}
          </ol>
        </div>
      );
    } else {
      return (
        <div style={ userDocsStyle }>No docs found...</div>
      );
    }
  };

  const onChange = address => {
    setAccountSelected(address);
  };

  return (
    <Grid.Column width={8}>
      <h1>Proof of Existence - User Info</h1>
      <Form>
        <Form.Field style={{ textAlign: 'center' }}>
        <Dropdown
            search
            selection
            clearable
            placeholder='Select an account to transfer'
            options={keyringOptions}
            onChange={(_, dropdown) => {
              onChange(dropdown.value);
            }}
            value={accountSelected}
          />

          <Button
            color='green'
            basic
            disabled={!accountSelected}
            onClick={queryUserDoc}
          >
            Query User Doc
          </Button>
        </Form.Field>
      </Form>
      {showingUserDocs && <UserDocs docs={userDocs}/>}
    </Grid.Column>
  );
}

export default function PoeModule (props) {
  const { api } = useSubstrate();
  return (api.query.poeModule && api.query.poeModule.proofs 
    ? <Main {...props} /> : null);
}
