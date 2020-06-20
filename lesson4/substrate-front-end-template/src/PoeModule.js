import React, { useEffect, useState } from 'react';
import { Form, Input, Grid } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';
import { blake2AsHex } from '@polkadot/util-crypto';
import { keyring } from '@polkadot/keyring';


function Main(props) {
  const { api } = useSubstrate();
  const { accountPair } = props;

  // The transaction submission status
  const [status, setStatus] = useState('');
  const [digest, setDigest] = useState('');
  const [owner, setOwner] = useState('');
  const [blockNumber, setBlockNumber] = useState(0);
  const [AccountId, setAccountId] = useState('');
  const [note, setNote] = useState('');
  const [claimlistAccountId, setclaimlistAccountId] = useState('');
  const [claimlist, setClaimlist] = useState([]);
  const [sucessinfo, setSucessinfo] = useState('');
  const convert = (from, to) => str => Buffer.from(str, from).toString(to)
  const utf8ToHex = convert('utf8', 'hex')
  const hexToUtf8 = convert('hex', 'utf8')
  let list_info  = []
  

  useEffect(() => {
    let unsubscribe;
    
    api.query.poeModule.proofs(digest, result => {
      setOwner(result[0].toString())
      //setBlockNumber(result[1].toNumber())
    }).then(unsub => {
      unsubscribe = unsub;
    })
      .catch(console.error);

    return () => unsubscribe && unsubscribe();
  }, [digest, api.query.poeModule]);

  const handleFileChosen = (file) => {
    const fileReader = new FileReader();

    const bufferToDigest = () => {
      const content = Array.from(new Uint8Array(fileReader.result))
        .map((b) => b.toString(16).padStart(2, '0'))
        .join('');

      const hash = blake2AsHex(content, 256);
      setDigest(hash)
    }

    fileReader.onloadend = bufferToDigest;
    fileReader.readAsArrayBuffer(file);
  }
  

  async function getUserDocs(acct) {
    setClaimlist('')
    list_info = []

    const unsub = await api.tx.poeModule.getClaimlist(claimlistAccountId).signAndSend(accountPair, (result) => {
        const {status, events} = result;
        console.log(status);
        console.log(events);

        if (result.status.isInBlock) {
          console.log(`Transaction included at blockHash ${result.status.asInBlock}`);
        } else if (result.status.isFinalized) {
          console.log(`Transaction finalized at blockHash ${result.status.asFinalized}`);


          // Loop through Vec<EventRecord> to display all events
          events.forEach(({ phase, event: { data, method, section } }) => {

            if ('ListOneClaim' === method) {
              list_info.push(data.toJSON())
            }
            if ('ExtrinsicSuccess' == method) {
              let tmp = "";
              tmp += "{";
              for (var index =0; index < list_info.length; index++) {
                let item = list_info[index];

                let note = hexToUtf8(item[4].substr(2));
                tmp += `${item[1]}=>(\"${note}\", ${item[3]}),`;
              }
              tmp += "}";
              setClaimlist(tmp);
            }
          });
          unsub();
        }

      }).catch(err => {

      });

  }

  return (
    <Grid.Column width={8}>
      <h1>Proofs of Existence Module</h1>
      <Form>
        <Form.Field>
          <Input
            type='file'
            id='file'
            label='Your File'
            onChange={(e) => handleFileChosen(e.target.files[0])}
          />
        </Form.Field>
        {/* <Form.Field>
          <Input
            label='transfer to AccountId'
            state='newValue'
            type='text'
            onChange={(_, { value }) => setAccountId(value)}
          />
        </Form.Field> */}
        <Form.Field>
          <Input
            label='claim note'
            state='newValue'
            type='text'
            maxLength="256"
            onChange={(_, { value }) => setNote(value)}
          />
        </Form.Field>
        <Form.Field>
          <TxButton
            accountPair={accountPair}
            label='Create Claim'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'createClaim',
              inputParams: [digest, note],
              paramFields: [true]
            }}
          />

          <TxButton
            accountPair={accountPair}
            label='Revoke Claim'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'revokeClaim',
              inputParams: [digest],
              paramFields: [true]
            }}
          />
          {/* <TxButton
            accountPair={accountPair}
            label='Transfer Claim'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: "poeModule",
              callable: "transferClaim",
              inputParams: [digest, AccountId],
              paramFields: [true],
            }}
          /> */}
        </Form.Field>

        {/* <div>{status}</div> */}
        {/* <div>{`claim info, owner: ${owner}, blockNumber: ${blockNumber}`}</div> */}
        <div>{`You have sucessfully claimed file with hash ${digest}  with note "${note}"`}</div>
      </Form>

      <Form>
      <Form.Field>
          <Input
            label='User Address'
            state='newValue'
            type='text'
            onChange={(_, { value }) => setclaimlistAccountId(value)}
          />
      </Form.Field>
      <Form.Field>
        <button onClick={getUserDocs}>Query User Doc
        </button>
      </Form.Field>
      
      <div>{claimlist}</div>
      </Form>
    </Grid.Column>
  );
}

export default function PoeModule(props) {
  const { api } = useSubstrate();
  return (api.query.poeModule && api.query.poeModule.proofs
    ? <Main {...props} /> : null);
}
