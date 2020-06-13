import React, { useEffect, useState } from 'react';
import { Form, Input, Grid, Card, Statistic } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';
import { blake2AsHex } from '@polkadot/util-crypto';

function Main (props) {
  const { api } = useSubstrate();
  const { accountPair } = props;

  // The transaction submission status
  const [status,setStatus] = useState('');
  const [digest,setDigest] = useState('');
  const [owner,setOwner] = useState('');
  const [blockNumber,setBlockNumber] = useState(0);
  const [dest,setDest] = useState('');
  // The currently stored value
  //const [currentValue, setCurrentValue] = useState(0);
  //const [formValue, setFormValue] = useState(0);

  useEffect(() => {
    let unsubscribe;
    api.query.poeModule.proofs(digest,(result) => {
      // The storage value is an Option<u32>
      // So we have to check whether it is None first
      // There is also unwrapOr
      // if (newValue.isNone) {
      //   setCurrentValue('<None>');
      // } else {
      //   setCurrentValue(newValue.unwrap().toNumber());
      // }
      setOwner(result[0].toString());
      setBlockNumber(result[1].toNumber());
    }).then(unsub => {
      unsubscribe = unsub;
    })
      .catch(console.error);

    return () => unsubscribe && unsubscribe();
  }, [digest,api.query.poeModule]);

  const handleFileChosen = (file) => {
    let fileReader = new FileReader();
    const bufferToDigest = () => {
      const content = Array.from(new Uint8Array(fileReader.result))
        .map((b) => {b.toString(16).padStart(2,'0')})
        .join('');
      const hash = blake2AsHex(content,256);
      setDigest(hash);
    }

    fileReader.onloadend = bufferToDigest;

    fileReader.readAsArrayBuffer(file);
  }
  const onDestChange = (value) => {
    setDest(value);
  }

  return (
    <Grid.Column width={8}>
      <h1>Proofs Module</h1>
      <Form>
        <h3>Proofs File</h3>
        <Form.Field>
          <Input
            id='file'
            label='file'
            type='file'
            placeholder='Your File'
            state=''
            onChange={(e)=>{handleFileChosen(e.target.files[0])}}/>
        </Form.Field>
        <h3>Transfer Proofs To</h3>
        <Form.Field>
          <Input
            fluid
            label='To'
            type='text'
            placeholder='Address'
            state='dest'
            onChange={(e)=>{onDestChange(e.target.value)}}
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
                inputParams:[digest],
                paramFields: [true]
              }}/>
          <TxButton
              accountPair={accountPair}
              label='Revoke Claim'
              setStatus={setStatus}
              type='SIGNED-TX'
              attrs={{
                palletRpc: 'poeModule',
                callable: 'revokeClaim',
                inputParams:[digest],
                paramFields: [true]
              }}/>
          <TxButton
              accountPair={accountPair}
              label='Transfer Claim'
              setStatus={setStatus}
              type='SIGNED-TX'
              attrs={{
                palletRpc: 'poeModule',
                callable: 'transferClaim',
                inputParams:[digest,dest],
                paramFields: [true]
              }}/>
        </Form.Field>
        <div>
          {status}
        </div>
        <div>
          `Claim info,owner:${owner},blockNumber:${blockNumber}`
        </div>
      </Form>
    </Grid.Column>
  );
}

export default function PoeModule (props) {
  const { api } = useSubstrate();
  return (api.query.poeModule && api.query.poeModule.proofs
    ? <Main {...props} /> : null);
}
