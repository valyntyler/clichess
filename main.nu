#!/usr/bin/env nu

def main [] {
  let url = {
    scheme: https
    host: lichess.org
    path: api/account
  }
  | url join

  let headers = {
    Authorization: $"Bearer ($env.LICHESS_API_TOKEN)"
  }

  http get $url -H $headers
  | explore
}
