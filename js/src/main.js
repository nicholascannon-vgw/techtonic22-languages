import express from 'express';
import { corsMiddleware, disableKeepAlive } from './middleware.js';

const app = express();
app.use(disableKeepAlive);
app.use(express.json({ limit: '1mb' }));
app.use(corsMiddleware);

app.get('/healthcheck', (_req, res) => {
    return res.json({ message: 'healthy' });
});

app.post('/count', (req, res) => {
    const { text } = req.body;
    const wordCounts = new Map();

    // TODO: Count words here...

    return res.json(Object.fromEntries(wordCounts.entries()));
});

const server = app.listen(8000, () => console.log('Service started on port 8000...'));
process.on('SIGINT', () => {
    console.log('Attempting to close server...');
    server.close(() => console.log('Server closed'));
});
