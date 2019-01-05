print('Player starting HP: ' .. player:hp())
print('Player starting Zeny: ' .. player:zeny())

print('Now lets add some zenies')

player:add_zeny(1)

print('Player current Zeny: ' .. player:zeny())

print('Now lets add some zenies')

player:add_zeny(10)

print('Player current Zeny: ' .. player:zeny())
