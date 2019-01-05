num = 42

if num > 40 then
  print('over 40')
end

playerId = 'foo'
playerHp = getPlayerHp(playerId)

print('Players current HP:' .. player():hp())
print('Players current HP:' .. player:hp())
print('Players current HP:' .. player())
