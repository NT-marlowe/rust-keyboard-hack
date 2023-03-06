import json

keyboard = input('Keyboard name: ')
dirname = keyboard
file_base_name = keyboard
json_file = open('./{}/{}.json'.format(dirname, file_base_name), 'r')
json_data = json.load(json_file)
json_file.close()

save_file_name = './{}/{}-hid-data.txt'.format(dirname, file_base_name)
save_file = open(save_file_name, 'w')
for d in json_data:
    hiddata = d['_source']['layers']['usbhid.data'].replace(':', '')
    save_file.write(hiddata+'\n')
save_file.close()
