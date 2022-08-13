import express from 'express';
import { corsMiddleware } from './middleware.js';

const app = express();
app.use(express.json());
app.use(corsMiddleware);

app.get('/healthcheck', (_req, res) => {
    return res.json({ message: 'healthy' });
});

app.post('/count', (req, res) => {
    const { text } = req.body;

    const wordCount = new Map();

    const words = text.split(' ');
    for (let word of words) {
        if (word.trim() === '') {
            continue;
        }
        if (word.includes(',')) {
            word = word.replace(',', '');
        }
        if (word.includes('!')) {
            word = word.replace('!', '');
        }

        const count = wordCount.get(word) || 0;
        wordCount.set(word, count + 1);
    }

    return res.json({ ...Object.fromEntries(wordCount) });
});

const server = app.listen(8000, () => console.log('Service started on port 8000...'));
process.on('SIGINT', () => server.close(() => console.log('Server closed')));
