bittrex_dl
==========

Download deposit and withdrawal histories from Bittrex exchange.

Setup
-----
Add a read-only api key to your Bittrex account from the settings
section of the exchange website.

Then create a YAML file in your home directory named *.bittrex_dl.yaml* and
populate it with the api key and secret.

    api:
      key: alladin
      secret: open-sesame

Deposits
--------

    bittrex_dl deposits > deposits.csv

Withdrawals
-----------

    bittrex_dl withdrawals > withdrawals.csv
