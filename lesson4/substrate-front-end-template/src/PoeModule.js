import React, { useEffect, useState, useCallback } from 'react';
import { Form, Input, Grid } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';
import { blake2AsHex } from '@polkadot/util-crypto';

function Main (props) {
  const { api } = useSubstrate();
  const { accountPair } = props;

  // The transaction submission status
  const [status, setStatus] = useState('');
  const [queryInfo, setQueryInfo] = useState('');
  const [digest, setDigest] = useState('');
  const [owner, setOwner] = useState('');
  const [user, setUser] = useState('');
  const [blockNumber, setBlockNumber] = useState(0);
  const [memo, setMemo] = useState('');

  function hex_to_ascii(str1) {
    var hex  = str1.toString();
    var str = '';
    for (var n = 0; n < hex.length; n += 2) {
      str += String.fromCharCode(parseInt(hex.substr(n, 2), 16));
    }
    return str;
  }

  useEffect(() => {
    let unsubscribe;
    api.query.poeModule.proofs(digest, (result) => {
      setOwner(result[1].toString());
      setBlockNumber(result[2].toNumber());
    }).then(unsub => {
      unsubscribe = unsub;
    })
      .catch(console.error);

    return () => unsubscribe && unsubscribe();
  }, [digest, api.query.PoeModule]);

  useEffect(() => {
    let unsubscribe;
    api.query.poeModule.proofByAcct(user, (result) => {
      setQueryInfo(result[0].toJSON());
    }).then(unsub => {
      unsubscribe = unsub;
    })
      .catch(console.error);

    return () => unsubscribe && unsubscribe();
  }, [digest, api.query.PoeModule]);

  const handleFileChosen = (file) => {
    let fileReader =  new FileReader();
    
    const bufferToDigest = () => {
      const content = Array.from(new Uint8Array(fileReader.result))
        .map((b) => b.toString(16).padStart(2, '0'))
        .join('');

      const hash = blake2AsHex(content, 256);
      setDigest(hash);
    };

    fileReader.onloadend = bufferToDigest;

    fileReader.readAsArrayBuffer(file);
  };

  return (
    <Grid.Column width={8}>
      <h1>Proof of Existence Module</h1>
      <Form>
        <Form.Field>
          <Input
            type='file'
            id='file'
            label='Your file:'
            onChange={ (e) => handleFileChosen(e.target.files[0]) }
          />
        </Form.Field>

        <Input
          label='Memo'
          placeholder='(no longer than 256 bytes)...'
          state='memo'
          type='string'
          onChange={(_, { value }) => setMemo(value)}
        />

        <Form.Field>
          <TxButton
            accountPair = {accountPair}
            label = 'Create Claim'
            setStatus = {setStatus}
            type = 'SIGNED-TX'
            attrs = {{
              palletRpc: 'poeModule',
              callable: 'createClaim',
              inputParams: [digest, memo],
              paramFields: [true]
            }}
          />
        </Form.Field>

        <div>{status}</div>
          <div>{`You have successfully claimed a file (Hash: ${digest}; Memo: ${memo}) in block ${blockNumber}.`}</div>
      </Form>

      <h1>Proof of Existence - User Info</h1>
      <Form>

        <Form.Field>

          <Input
            label='User Address'
            placeholder='0x123abc...'
            state='user'
            type='string'
            onChange={(_, { value }) => setUser(value)}
          />

          <TxButton
            accountPair={accountPair}
            label='Query user doc'
            setStatus={setQueryInfo}
            type='QUERY'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'proofByAcct',
              inputParams: [user],
              paramFields: [true]
            }}
          />

        </Form.Field>

        <div id="docinfo">{`Doc Info. ${queryInfo}`}</div>
      </Form>

    </Grid.Column>
  );
}

export default function PoeModule (props) {
  const { api } = useSubstrate();
  return (api.query.poeModule && api.query.poeModule.proofs
    ? <Main {...props} /> : null);
}
