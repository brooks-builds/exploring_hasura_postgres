const bodyParser = require('body-parser');
const express = require('express');
const jsonWebToken = require('jsonwebtoken');
const app = express();
const jsonTokenSecret = process.env.JSON_TOKEN_SECRET || 'keyboardcatkeyboardcatkeyboardcatkeyboardcat';

app.use(bodyParser.json());
app.use(bodyParser.urlencoded());

app.get('/', (req, res) => res.json({ "message": "Hi there" }));
const expectedPassword = '1234';


// we need to get this working by looking at the Hasura documentation
// https://hasura.io/docs/latest/graphql/core/auth/authentication/jwt.html#
app.post('/login', (req, res) => {
    let username = req.body.input.username;
    let password = req.body.input.password;

    console.log(username, password);
    if (password != expectedPassword) return res.status(401).json({ error: `password incorrect: ${password}`, token: null });
    res.header('x-hasura-role', 'user');
    res.json({
        token: jsonWebToken.sign({
            username, "https://hasura.io/jwt/claims": {
                'x-hasura-allowed-roles': ['user'],
                'x-hasura-default-role': 'user',
                'x-hasura-user-id': '1',
                'x-hasura-org-id': '1',
            }
        }, jsonTokenSecret),
        error: null
    });
});

app.listen(3000, () =>
    console.log('app listening on port 3000')
);

// eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VybmFtZSI6ImIiLCJpYXQiOjE2Mzc1MjIxODJ9.qWy86FskfeXViLi5F_hVT6eunsmF73dxJFN9qkqu9AA