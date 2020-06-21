import React, { useEffect, useState } from 'react';
import { Form, Input, Grid, Card, Statistic, Label, TextArea } from 'semantic-ui-react';

import { useSubstrate } from './substrate-lib';
import { TxButton } from './substrate-lib/components';
import { blake2AsHex } from '@polkadot/util-crypto';

function Main (props) {
  const { api } = useSubstrate();
  const { accountPair } = props;

  // The transaction submission status
  const [status, setStatus] = useState('');
  const [digest, setDigest] = useState('');
  const [owner, setOwner] = useState('');
  const [blockNumber, setBlockNumber] = useState('');
  const [AccountId, setAccountId] = useState('');
  // for lesson4 homework
  const [dest, setDest] = useState('');
  const [amount, setAmount] = useState(0);
  const [note, setNote] = useState('');
  const [showingNotification, setShowingNotification] = useState(false);

  useEffect(() => {
    let unsubscribe;
    api.query.poeModule.proofs(digest, (result) => {
      setOwner(result[0].toString());
      setBlockNumber(result[1].toNumber());
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
  };

  // for lesson4 homework
  const onDestChange = (_, data) => {
    setDest(data.value);
  };

  const onAmountChange = (_, data) => {
    setAmount(data.value);
  };

  const MAX_NOTE_LENGTH = 256;
  const onNoteChange = (_, data) => {
    if(data.value && data.value.length > MAX_NOTE_LENGTH) {
      data.value = data.value.substring(0, MAX_NOTE_LENGTH);
    }
    setNote(data.value);
  };

  const setExtrinsicStatus = (data) => {
    console.log(data);
    console.log(data.indexOf('Finalized'));
    if(data.indexOf('Finalized') !== -1) {
      setShowingNotification(true);
      setTimeout(() => setShowingNotification(false), 20000);
    }
    setStatus(data);
  };

  const SuccessNotification = (props) => {
    const { digest, note } = props;
    const notificationStyle = {
      marginTo: 10,
      border: '1px solid green',
      backgroundColor: 'lightgreen',
      color: 'darkgreen',
      borderRadius: 5,
      padding: 10
    };
    return (
      <div style={notificationStyle}>
        You have successfully claimed file with hash <strong>{digest}</strong> with note <strong>"{note}"</strong>.
      </div>
    );
  };

  return (
    <Grid.Column width={8}>
      <h1>Proof of Existence Module</h1>
     <Form>
       <Form.Field>
         <Input 
          type='file'
          id='file'
          label='Your file'
          onChange={ (e) => handleFileChosen(e.target.files[0])}
          />
       </Form.Field>
       <Form.Field>
         <Label>Note</Label>
         <TextArea
          type='text'
          placeholder='Som note( max 256 chars)'
          state='note'
          maxLength={256}
          onChange={onNoteChange}
          />
       </Form.Field>
       <Form.Field>
         <Input
          type='text'
          label='To'
          placeholder='address'
          state='dest'
          onChange={onDestChange}
          />
       </Form.Field>
       <Form.Field>
         <Input
          fluid
          label='Amount'
          type='number'
          state='amount'
          onChange={onAmountChange}
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
            inputParams: [digest],
            paramFields: [true],
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
                paramFields: [true],
              }}
          />
       </Form.Field>
       <Form.Field>
         <Input
            label='transfer to AccountId'
            state='newValue'
            type='text'
            onChange={(_, {value}) => setAccountId(value)} 
         />
       </Form.Field>
       <Form.Field>
         <TxButton
          accountPair={accountPair}
          label='Transfer Claim'
          setStatus={setStatus}
          type='SIGNED-TX'
          attrs={{
            palletRpc: 'poeModule',
            callable: 'transferClaim',
            inputParams: [digest, AccountId],
            paramFields: [true],
          }}
         />
       </Form.Field>
       {showingNotification && <SuccessNotification digest={digest} note={note}/>}
       <div>{status}</div>
       <div>['Claim info, owner: ${owner}, blockNumber: ${blockNumber}']</div>
    </Form>    
    </Grid.Column>
  );
}

export default function PoeModule (props) {
  const { api } = useSubstrate();
  return (api.query.poeModule && api.query.poeModule.proofs
    ? <Main {...props} /> : null);
}
