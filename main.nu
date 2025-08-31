#!/usr/bin/env nu

def "chat read" [
  game_id: string
] {
  let url = {
    scheme: https
    host: lichess.org
    path: api/board/game/($game_id)/chat
  } | url join

  let headers = {Authorization: $"Bearer ($env.LICHESS_API_TOKEN)"}

  (
    http get $url
    --headers $headers
  )
}

def "chat send" [
  game_id: string
  message: string
] {
  let url = {
    scheme: https
    host: lichess.org
    path: api/board/game/($game_id)/chat
  } | url join

  let msg = {
      room: "player"
      text: $message
  }

  let headers = {Authorization: $"Bearer ($env.LICHESS_API_TOKEN)"}

  (
    http post $url $msg
    --headers $headers
    --content-type "application/x-www-form-urlencoded"
  )
}

def main [game_id: string] {
  chat read $game_id
  chat send $game_id "oeugh"
  chat read $game_id
}
