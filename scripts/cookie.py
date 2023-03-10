import sys
import time

import undetected_chromedriver as uc

def get_cookie(driver):
    cookies = driver.get_cookies()
    for c in cookies:
        if c.get("name") == "cf_clearance":
            return c.get("value")
    return ""


def main(param):
    print("open: " + param[0], file=sys.stderr)
    driver = uc.Chrome(use_subprocess=True, version_main=108)
    try:
        with driver:
            driver.get(param[0])
            cf_cookie = get_cookie(driver)
            c = 0
            while not cf_cookie:
                c += 1
                if c > 10:
                    print("no cookie", file=sys.stderr)
                    return 1
                print("sleep", file=sys.stderr)
                time.sleep(1)
                cf_cookie = get_cookie(driver)
            print("got", cf_cookie, file=sys.stderr)
            print(cf_cookie)
    finally:
        driver.quit()
    print("done", file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
