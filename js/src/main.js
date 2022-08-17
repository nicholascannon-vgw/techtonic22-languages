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

    // TODO: implementation
});

const server = app.listen(8000, () => console.log('Service started on port 8000...'));
process.on('SIGINT', () => server.close(() => console.log('Server closed')));
