const express = require('express');
const app = express();

// Middleware para parsear JSON
const PORT = process.env.PORT || 3000;
app.use(express.json());

// This is Head X-Forwarded-For send reverse-proxy info
// For parser that data and send response

app.use((req, res, next) => {
  const clientIp = req.headers['x-forwarded-for'] || req.socket.remoteAddress;
  res.setHeader('X-Forwarded-For', clientIp);
  res.setHeader('X-Request-Method', req.method);
  res.setHeader('X-Service-Name', 'Express');
  res.setHeader('X-Response-Timestamp', new Date().toISOString());
  next();
});

// Endpoint raíz
app.get('/', (req, res) => {
  res.json({ message: '¡Servidor Express corriendo con Node.js!' });
});

app.listen(PORT, () => {
  console.log(`Servidor escuchando en http://localhost:${PORT}`);
});
