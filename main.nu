#!/usr/bin/env nu

def main [gameId: string] {
  let url = {
    scheme: https
    host: lichess.org
    path: api/board/game/($gameId)/chat
  } | url join

  let msg = {
      room: "player"
      text: "hello!"
  }

  let headers = {Authorization: $"Bearer ($env.LICHESS_API_TOKEN)"}

  (
    http post $url $msg
    --headers $headers
    --content-type "application/x-www-form-urlencoded"
  )
}
