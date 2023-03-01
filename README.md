# Read keyboard input using **rusb** crate!   
## Tips
* Real time dmesg: ```sudo dmesg -e -w```
* Show verbose lsusb: ```sudo lsusb -d ****:**** -v | less```
* Re-connect usb device: ```sudo sh -c "echo -n *-* > /sys/bus/usb/drivers/usb/unbind && echo -n *-* > /sys/bus/usb/drivers/usb/bind"```