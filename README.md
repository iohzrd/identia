# identia

Decentralized, censorship resistant social media on IPFS. Plus experimental RSS reader.

Successor to [follow](https://github.com/iohzrd/follow)

![](screenshot1.png)
![](screenshot2.png)
![](screenshot3.png)

## Development / Running from source

Follow [this guide](https://tauri.studio/v1/guides/getting-started/prerequisites) to setup dev dependencies...

### Linux

For video playback on linux, you may need to manually install gstreamer/gst-plugins:
`gstreamer gst-libva gst-plugins-bad gst-plugins-base gst-plugins-good gst-plugins-ugly`

For hardware video acceleration install: `gstreamer-vaapi`

```
npm install
npm start
```

## TODO

- [x] prototype logic
- [x] periodically re-publish self identity
- [x] periodically update identities you follow...
- [x] cache posts
- [x] enable file in posts
- [x] bundle IPFS binaries and manage execution
- [x] post view
- [x] implement tray.
- [x] progress events for spinners and such
- [x] Distribute binaries
- [x] migrate to SQLite...
- [x] Paginate feed
- [x] enable(fix) audio/video playback
- [ ] "re-post" / mirror a post
- [ ] re-implement comment system via pubsub
- [ ] include crypto signature of body in post
- [ ] implement private messaging with libsignal or [DiscoCrypto](https://discocrypto.com/#/)
- [ ] mechanism for exporting / importing, identity / posts
- [ ] strip exif data from images
- [ ] include index.html with posts to allow styling for browser consumption...
- [ ] Auto-update system
- [ ] advanced pin management
- [ ] meta view
- [ ] settings view
- [ ] keybase-like functionality...
- [ ] mobile app...
- [ ] re-imlpement tor hidden service
- [ ] serve web-frendly html with identity + TOR

## Community:

```
Matrix:
#general:identia.io

Discord:
https://discord.gg/aErhA6TBPS
```

## Follow me

```
12D3KooWHxU85q4JWsDXq4ZHjBCdjHHGL9wnMtqBMMgArkn6xcyz
```

## Support me

```
BTC:
bc1qtss3kxqcfl7glaskys7tevs98e6v62rhpx8lzj
```

or

https://www.patreon.com/iohzrd

## License

[AGPL-3.0](LICENSE)
