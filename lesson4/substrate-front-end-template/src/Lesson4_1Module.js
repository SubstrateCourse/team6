import React, { useEffect, useState } from 'react';
import { Form, Input, Grid, Label, TextArea } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';
import { blake2AsHex} from '@polkadot/util-crypto';

function Main (props) {
  const { api, keyring } = useSubstrate();
  //const { api } = useSubstrate();
  const { accountPair } = props;

  // The transaction submission status
  const [status, setStatus] = useState('');
  const [digest, setDigest] = useState('');
  const [owner, setOwner] = useState('');
  const [remarkInfo,setRemarkInfo] = useState(0);
  const [blockNumber, setBlockNumber] = useState(0);
  const [proofValue, getProofValue] = useState(0);
  const [remarkInfoResult,setRemarkInfoResult] = useState(0);

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

  const handleFileChosen = (file) => {
    let fileReader = new FileReader();

    const bufferToDigest = () => {
      const content = Array.from(new Uint8Array(fileReader.result))
        .map((b) => b.toString(16).padStart(2, '0'))
        .join('');

      const hash = blake2AsHex(content, 256);
      setDigest(hash);
    }

    fileReader.onloadend = bufferToDigest;

    fileReader.readAsArrayBuffer(file);
  }

  const MAX_NOTE_LENGTH = 256;
  const onNoteChange = (_, data) => {
    if (data.value && data.value.length > MAX_NOTE_LENGTH) {
      data.value = data.value.substring(0, MAX_NOTE_LENGTH);
    }
    setRemarkInfo(data.value);
  };

  const notificationStyle = {
    marginTop: 10,
    border: '1px solid green',
    backgroundColor: 'lightgreen',
    color: 'darkgreen',
    borderRadius: 5,
    padding: 10
  };

  // Get the list of accounts we possess the private key for
  const keyringOptions = keyring.getPairs().map(account => ({
    key: account.address,
    value: account.address,
    text: account.meta.name.toUpperCase(),
    icon: 'user'
  }));

  return (
    <Grid.Column width={8}>
      <h1>Proof of Existence</h1>
      <Form>
        <Form.Field>
          <Input
            type='file'
            id='file'
            lable='Your File'
            onChange={ (e) => handleFileChosen(e.target.files[0]) }
          />
        </Form.Field> 

        <Form.Field>
          <Label>Note</Label>
          <TextArea
            type='text'
            placeholder='Please input  noting (max limit 256 chars)'
            state='Note'
            maxLength={256}
            onChange={onNoteChange}
          />
        </Form.Field>

        <Form.Field style={{ textAlign: 'center' }}>
          <TxButton
            accountPair = {accountPair}
            label='Create Claim'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'createClaim',
              inputParams: [digest, 100, remarkInfo],
              paramFields: [true]
            }}
          />
        </Form.Field>

        <div style={notificationStyle}>{ `You have successfully claimed file with hash  ${digest} with note "${remarkInfo}" ` }</div>
        <div>{status}</div>
      </Form>
    </Grid.Column>
  );
}

export default function PoeModule (props) {
  const { api } = useSubstrate();
  return (api.query.poeModule && api.query.poeModule.proofs 
    ? <Main {...props} /> : null);
}
