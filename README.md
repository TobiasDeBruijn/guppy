# Guppy
Simple tool to upload files to Google Drive using a service account

# Setup
1. Create a project in [Google Developer Console](https://console.developers.google.com)
2. Enable the Google Drive API
3. Create a Service account
    1. Create a key in the service account, of type JSON
    2. From the key file, copy the value for `private_key`, and replace all `\n` characters with a newline. 
    >Note: Make sure not to alter the file besides this.
    3. Paste the result in a text file.
4. Launch guppy:
```
guppy --pem <Path to the text file> --email <value of `client_email`> upload --folder <Optional folder ID> --team-drive <Optional Team Drive ID> <File>
```

## Scripts
I wrap Guppy further with helper scripts, these, and an explanation about how these work, can be found [here](scripts/README.md)

# License
This project is available under the MIT or Apache 2.0 license, at your option.