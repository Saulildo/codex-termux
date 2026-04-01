---
name: termux-tts
description: Use Android text-to-speech from Termux via termux-tts-speak. Use when the user wants spoken output or when validating Termux TTS integration on-device.
---

Use this skill only on Termux-capable Android devices.

Workflow:
- Check `command -v termux-tts-speak` before trying to speak.
- If the command is missing, state that `Termux:API` and the `termux-api` package are required.
- For a normal request, run `termux-tts-speak "<text>"`.
- Keep the spoken text short unless the user explicitly asks for a longer message.
- After running the command, report whether it succeeded and include the exact shell command used.

Notes:
- This skill is Termux-specific and should not be used on non-Android hosts.
- Prefer the default voice/settings unless the user explicitly asks for a different engine, language, pitch, or rate.
