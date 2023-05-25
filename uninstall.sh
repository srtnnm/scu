#!/bin/bash

if [[ "$(id -u)" -eq 0 ]]; then
  if [[ -f "/usr/bin/scu" ]]; then
    rm /usr/bin/scu
  else
    echo "SCU was not installed previously."
  fi
else
  echo "Please run this script using sudo"
  echo "sudo bash uninstall.sh"
fi
