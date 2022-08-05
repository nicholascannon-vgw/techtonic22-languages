const express = require('express');
const { isEmptyString, notString } = require('./validation');

const app = express();
app.use(express.json());

app.get('/healthcheck', (_req, res) => {
    return res.json({ message: 'healthy' });
});

app.post('/count', (req, res) => {
    const { text } = req.body;
    if (notString(text) || isEmptyString(text)) {
        return res.sendStatus(400);
    }

    const wordCount = new Map();

    const words = text.split(' ');
    for (const word of words) {
        // replace all punctuation
        const sanitizedWord = word.replace(/[.,\/#!$%\^&\*;:{}=\-_`~()]/g, '');
        if (isEmptyString(word)) {
            continue;
        }

        const count = wordCount.get(sanitizedWord) || 0;
        wordCount.set(sanitizedWord, count + 1);
    }

    return res.json({ wordCount: Object.fromEntries(wordCount) });
});

const server = app.listen(8000, () => console.log('Service started on port 8000...'));
process.on('SIGINT', () => server.close(() => console.log('Server closed')));
