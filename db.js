<!-- NODE JS-->
const express = require('express');
const app = express();
const bodyParser = require('body-parser');

app.use(bodyParser.json());
const db = {};

app.get('/get', (req, res) => {
    res.json(db);
});

app.post('/insert', (req, res) => {
    const { key, value } = req.body;
    db[key] = value;
    res.json({ message: 'Row inserted successfully.' });
});

app.post('/delete', (req, res) => {
    const { key } = req.body;
    if (key in db) {
        delete db[key];
        res.json({ message: 'Row deleted successfully.' });
    } else {
        res.json({ error: 'Key not found.' });
    }
});

app.listen(3000, () => {
    console.log('Server started on port 3000');
});
