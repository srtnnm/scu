from urllib.request import urlretrieve
import os

PATH_TO_SOURCES = '/'.join(__file__.split('/')[0:-1])

GPU_VENDORS = ["10de","1002","8086","1a03","15ad","1af4"]

PCI_IDS_FILENAME = "pci.ids"
PCI_IDS_COMMON_PATHS = [
    "/usr/share/hwdata",
    "/usr/share/misc",
    "/var/lib/pciutils"
]
PCI_IDS_DB_REPO_URL = "https://pci-ids.ucw.cz/v2.2/pci.ids"
PCI_IDS_RS = f"{PATH_TO_SOURCES}/src/scu/pci_ids.rs"
PCI_IDS_RS_CONTENT = """#![cfg(feature = "pci_ids")]

pub fn contains(id: &str) -> bool {
    IDS.iter().any(|&(k, _)| id == k)
}

pub fn get(id: &str) -> Option<&str> {
    IDS.iter().find(|(k, _)| &id == k).map(|&(_, v)| v)
}

const IDS: [(&str, &str); %ids_len%] = [
\t%ids%
];
"""

CLEAN_BUF = ("","")


def parse_db(path: str) -> [(str,str)]:
    result = []

    vendor_buf = CLEAN_BUF
    device_buf = CLEAN_BUF
    subsystem_buf = CLEAN_BUF

    with open(path, 'r') as f:
        for line in f.read().split("\n"):
            if not line or line.strip()[0] in ['#','C']:
                continue

            if line.startswith("\t\t"):
                if vendor_buf[0] in GPU_VENDORS:
                    subsystem_buf = tuple(line.strip().split("  "))
                else:
                    subsystem_buf = CLEAN_BUF
            elif line.startswith("\t"):
                if vendor_buf[0] in GPU_VENDORS:
                    device_buf = tuple(line.strip().split("  "))
                    subsystem_buf = CLEAN_BUF
                else:
                    device_buf = CLEAN_BUF
            else:
                vendor_buf = tuple(line.strip().split("  "))
                device_buf = CLEAN_BUF

            if vendor_buf[0] in GPU_VENDORS:
                buf = vendor_buf[0]
                buf_name = ""
                if device_buf[0]:
                    buf+=":"+device_buf[0]
                if subsystem_buf[0]:
                    buf+=" "+subsystem_buf[0].replace(" ",":")
                for name in [subsystem_buf[1], device_buf[1],vendor_buf[1]]:
                    if name:
                        buf_name=name
                        break
                result.append((buf,buf_name.replace('"', '\\"')))

    return result


def download_pci_ids(filename: str) -> bool:
    try:
        print(f"Trying to download the latest {PCI_IDS_FILENAME} database from {PCI_IDS_DB_REPO_URL}")
        urlretrieve(PCI_IDS_DB_REPO_URL, filename)
        print("Successfully downloaded")
        return True
    except Exception as e:
        print(f"Error downloading. error: {e}\nMaybe you have no internet connection ?")
        return False


def search_pci_ids_in_system() -> str:
    for path in PCI_IDS_COMMON_PATHS:
        path = os.path.join(path, PCI_IDS_FILENAME)
        if os.path.isfile(path):
            print(f"{PCI_IDS_FILENAME} found in system - {path}")
            return path
    
    return ""


def gen_pci_ids_rs(ids):
    ids_to_file_content = ""
    for id in ids:
        ids_to_file_content += f"\t(\"{id[0]}\", \"{id[1]}\"),\n"

    return PCI_IDS_RS_CONTENT.replace("%ids_len%", str(len(ids))).replace("%ids%", ids_to_file_content.strip())


def main():
    pci_ids_path = search_pci_ids_in_system()

    if not pci_ids_path:
        print("File {PCI_IDS_FILENAME} not found in system")
        if download_pci_ids(PCI_IDS_FILENAME):
            pci_ids_path = PCI_IDS_FILENAME
        else:
            exit(-1)
    
    print(f"Parsing {pci_ids_path} ...")
    ids = parse_db(pci_ids_path)

    if len(ids)<1000:
        print(f"Parsed pci.ids (only GPU vendors) is too small, only {len(ids)} entries\n{PCI_IDS_RS} not updated")
        exit(-1)
    else:
        filecontent = gen_pci_ids_rs(ids)
        if not filecontent:
            print(f"I don't know why\nBut generated pci_ids.rs is empty\n{PCI_IDS_RS} not updated")
            exit(-1)
        else:
            try:
                open(PCI_IDS_RS,'w').write(filecontent)
                print(f"{PCI_IDS_RS} successfully updated")
            except Exception as e:
                print(f"{PCI_IDS_RS} failed to rewrite\nError: {e}")


if __name__ == "__main__":
    main()