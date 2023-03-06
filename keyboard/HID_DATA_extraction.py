import json

keyboard = input('Keyboard name: ')
dirname = keyboard
file_base_name = keyboard
with open('./{}/{}.json'.format(dirname, file_base_name), 'r') as json_file:
    json_data = json.load(json_file)

with open('./{}/{}-keys.txt'.format(dirname, file_base_name), 'r') as keys_file:
    keys = [line.rstrip() for line in keys_file]

save_file_name = './{}/{}-hid-data.txt'.format(dirname, file_base_name)
save_file = open(save_file_name, 'w')
for i in range(len(json_data)):
    jd = json_data[i]
    hiddata = jd['_source']['layers']['usbhid.data'].split(':')
    for hd in hiddata:
        save_file.write(format(int(hd), '08b'))
    save_file.write(',{}\n'.format(keys[i]))
save_file.close()
