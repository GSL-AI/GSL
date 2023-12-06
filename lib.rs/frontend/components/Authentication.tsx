import React, { useState } from 'react';
import { useNear } from 'near-api-js';
import { ZeroEx } from '0x.js';

const Authentication = () => {
    const [email, setEmail] = useState('');
    const [password, setPassword] = useState('');
    const [loading, setLoading] = useState(false);
    const near = useNear();

    const authenticate = async () => {
        setLoading(true);
        const zeroEx = new ZeroEx('https://api.0x.org');
        const signature = await zeroEx.order.getSignature(email, password);
        const accountId = await near.connection.signer.getAccountId();
        const result = await near.contract.authenticate({ accountId, signature });
        setLoading(false);
        console.log(result);
    };

    return (
        <div>
            <h1>Authentication Portal</h1>
            <form onSubmit={authenticate}>
                <label>
                    Email:
                    <input type="email" value={email} onChange={(e) => setEmail(e.target.value)} />
                </label>
                <br />
                <label>
                    Password:
                    <input type="password" value={password} onChange={(e) => setPassword(e.target.value)} />
                </label>
                <br />
                <button type="submit" disabled={loading}>
                    {loading ? 'Loading...' : 'Authenticate'}
                </button>
            </form>
        </div>
    );
};

export default Authentication;
