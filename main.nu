#!/usr/bin/env nu

use chat.nu

def main [game_id: string] {
  chat send $game_id "hiii"
  chat read $game_id
}
