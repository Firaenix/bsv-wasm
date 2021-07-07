const express = require('express');
const app = express()
const port = 3000
const path = require('path')

app.get('*', (req, res) => {
  console.log("Request path", req.path)
  if (req.path === '/') {
    return res.sendFile(__dirname + '/index.html')
  }

  return res.sendFile(path.join(__dirname, '../../pkg/web/' + req.path))
})

app.listen(port, () => {
  console.log(`Example app listening at http://localhost:${port}`)
})