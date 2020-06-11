import React, { useEffect, useState } from 'react';
import { Form, Input, Grid, Dropdown } from 'semantic-ui-react';

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
  //const [AccountId, setAccountId] = useState('5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty');
  const [accountSelected, setAccountSelected] = useState('');
  const [blockNumber, setBlockNumber] = useState(0);
  const [formValue, setFormValue] = useState(0);
  const [proofValue, setProofValue] = useState(0);

  useEffect(() => {
    let unsubscribe;
    api.query.poeModule.proofs(digest,(result) => {
      setOwner(result[0].toString());
      setBlockNumber(result[1].toNumber());
      setProofValue(result[2].toNumber());
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

  // Get the list of accounts we possess the private key for
  const keyringOptions = keyring.getPairs().map(account => ({
    key: account.address,
    value: account.address,
    text: account.meta.name.toUpperCase(),
    icon: 'user'
  }));

  const onChange = address => {
    setAccountSelected(address);
  };

  return (
    <Grid.Column width={8}>
      <h1>Proof of Existence Module</h1>
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
          <Input
            label='Price'
            state='newValue'
            type='number'
            onChange={(_, { value }) => setFormValue(value)}
          />
        </Form.Field>

        <Form.Field>
          <TxButton
            accountPair = {accountPair}
            label='Create Claim'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'createClaim',
              inputParams: [digest, formValue],
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
              inputParams: [digest, formValue],
              paramFields: [true]
            }}
          />

          <TxButton
            accountPair={accountPair}
            label='Buy Claim'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'buyClaim',
              inputParams: [digest, formValue],
              paramFields: [true]
            }}
          />

          <TxButton
            accountPair={accountPair}
            label='Buy Claim'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'buyClaim',
              inputParams: [digest, formValue],
              paramFields: [true]
            }}
          />

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

          <TxButton
            accountPair={accountPair}
            label='Transfer Claim'
            setStatus={setStatus}
            type='SIGNED-TX'
            attrs={{
              palletRpc: 'poeModule',
              callable: 'transferClaim',
              inputParams: [digest,accountSelected, formValue],
              paramFields: [true]
            }}
          />
        </Form.Field>

        <div>{status}</div>
            <div>{ `Claim info:owner: ${owner} ` }</div>
            <div>{`blockNumber: ${blockNumber}`}</div>
            <div>{`transferValue: ${proofValue}`}</div>
      </Form>
    </Grid.Column>
  );
}

export default function PoeModule (props) {
  const { api } = useSubstrate();
  return (api.query.poeModule && api.query.poeModule.proofs 
    ? <Main {...props} /> : null);
}
