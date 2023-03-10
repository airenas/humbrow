import sys
import time

import undetected_chromedriver as uc

def main(params):
    print("open: " + params[0], file=sys.stderr)
    driver = uc.Chrome(use_subprocess=True, version_main=108)
    try:
        with driver:
            driver.get(params[0])
            print("lets wait a bit", file=sys.stderr)
            elem = driver.find_element("xpath", "//*")
            source_code = elem.get_attribute("outerHTML")
            print(source_code, file=sys.stderr)
    finally:
        driver.quit()    
    print("done", file=sys.stderr)


if __name__ == "__main__":
    main(sys.argv[1:])
