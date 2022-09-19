import React, { useState, useEffect } from 'react'
import { Form, Grid } from 'semantic-ui-react'

import { useSubstrateState } from './substrate-lib'
import { TxButton } from './substrate-lib/components'
import KittyCards from './KittyCards'

export default function Kitties (props) {

  const { api, keyring } = useSubstrateState();
  const [status, setStatus] = useState("");
  const [kitties, setKitties] = useState([]);
  const [kittyDNAs, setKittyDNAs] = useState([]);
  const [kittyOwners, setKittyOwners] = useState([]);

  const fetchKitties = () => {
    let unsubscribe;
    api.query.kitties
      .nextKittyId((cnt) => {
        if (cnt !== "") {
          const kittyIds = Array.from(Array(parseInt(cnt, 10)), (v, k) => k);

          api.query.kitties.kittyOwner
            .multi(kittyIds, (kittyOwners) => {
              setKittyOwners(kittyOwners);
            })
            .catch(console.error);

          api.query.kitties.kitties
            .multi(kittyIds, (kittyDna) => {
              setKittyDNAs(kittyDna);
            })
            .catch(console.error);
        }
      })
      .then((unsub) => {
        unsubscribe = unsub;
      })
      .catch(console.error);

    return () => unsubscribe && unsubscribe();
  };

  const wrapKitties = () => {
    const kitties = [];
    for (let i = 0; i < kittyDNAs.length; ++i) {
      const kitty = {};
      kitty.id = i;
      kitty.dna = kittyDNAs[i].unwrap();
      kitty.owner = keyring.encodeAddress(kittyOwners[i].unwrap());
      kitties[i] = kitty;
    }
    setKitties(kitties);
  };

  useEffect(fetchKitties, [api, keyring]);
  useEffect(wrapKitties, [keyring, kittyDNAs, kittyOwners]);

  return <Grid.Column width={16}>
    <h1>小毛孩</h1>
    <KittyCards kitties={kitties} setStatus={setStatus}/>
    <Form style={{ margin: '1em 0' }}>
      <Form.Field style={{ textAlign: 'center' }}>
        <TxButton
          label='创建小毛孩'
          type="SIGNED-TX"
          setStatus={setStatus}
          attrs={{
            palletRpc: 'kitties',
            callable: 'create',
            inputParams: [],
            paramFields: []
          }}
        />
      </Form.Field>
    </Form>
    <div style={{ overflowWrap: 'break-word' }}>{status}</div>
  </Grid.Column>

}
