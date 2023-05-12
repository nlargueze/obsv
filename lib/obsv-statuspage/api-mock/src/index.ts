import express, { Request, Response } from 'express';
import cors from 'cors';
import db from './db';

const app = express();
app.use(cors())
const port = 3000;

app.get('/', (req: Request, res: Response) => {
	res.send('Hello World!');
});

app.get('/up', (req: Request, res: Response) => {
	res.send('UP');
});

const STATUS = db.genStatus();

app.get('/status', (req: Request, res: Response) => {
	console.log('-> received request');
	res.send(STATUS);
});

app.listen(port, () => {
	console.log(`Mock API listening on port ${port}`);
});
