export const corsMiddleware = (_req, res, next) => {
    res.setHeader('Access-Control-Allow-Origin', '*');
    res.setHeader('Access-Control-Allow-Methods', 'GET,POST,PUT,DELETE,OPTIONS');
    res.setHeader('Access-Control-Allow-Headers', 'Content-Type, Access-Control-Allow-Headers');
    next();
};

export const disableKeepAlive = (_req, res, next) => {
    res.setHeader('Connection', 'close');
    next();
};
