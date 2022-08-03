const express = require('express');
const { isEmptyString, notString } = require('./validation');
const { DICTIONARY } = require('./dictionary');

const app = express();
app.use(express.json());

app.get('/healthcheck', (_req, res) => {
    return res.json({ message: 'Spell checker service', status: 'healthy' });
});

app.post('/check', (req, res) => {
    const { text } = req.body;
    if (notString(text) || isEmptyString(text)) {
        return res.sendStatus(400);
    }

    const mistakes = [];

    const words = text.split(' ');
    for (const word of words) {
        const sanitizedWord = word.replace(/[.,\/#!$%\^&\*;:{}=\-_`~()]/g, '');
        if (isEmptyString(word)) {
            continue;
        }

        if (!DICTIONARY.has(sanitizedWord.toLowerCase())) {
            mistakes.push(sanitizedWord);
        }
    }

    return res.json({ mistakes });
});

const server = app.listen(8000, () => console.log('Service started on port 8000...'));
process.on('SIGINT', () => server.close(() => console.log('Server closed')));
