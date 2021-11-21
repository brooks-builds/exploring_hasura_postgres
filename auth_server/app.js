const bodyParser = require('body-parser');
const express = require('express');
const jsonWebToken = require('jsonwebtoken');
const app = express();
const jsonTokenSecret = process.env.JSON_TOKEN_SECRET || 'keyboardcat';

app.use(bodyParser.json());

app.get('/', (req, res) => res.json({ "message": "Hi there" }));


// we need to get this working by looking at the Hasura documentation
// https://hasura.io/docs/latest/graphql/core/auth/authentication/jwt.html#
app.post('/login', (req, res) => {
    const { username, password } = req.body.input;
    res.json({
        token: jsonWebToken.sign({
            username, "https://hasura.io/jwt/claims": {
                'x-hasura-allowed-roles': ['user'],
                'x-hasura-default-role': 'user'
            }
        }, jsonTokenSecret)
    });
});

app.listen(3000, () =>
    console.log('app listening on port 3000')
);

// eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6ImIiLCJpYXQiOjE2Mzc1MjIxODJ9.qWy86FskfeXViLi5F_hVT6eunsmF73dxJFN9qkqu9AA