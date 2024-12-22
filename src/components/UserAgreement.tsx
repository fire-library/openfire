import Dialog from "src/components/Dialog";
import { open } from "@tauri-apps/plugin-shell";
import { useUserAgreement } from "../tabs/userAgreementProvider";
import Success from "src/components/Button/Success";

export default function UserAgreement() {
  const { agreed, agree } = useUserAgreement();

  if (agreed === false)
    return (
      <Dialog title="End User Agreement" open={!agreed}>
        <div className="max-w-xl flex flex-col gap-4">
          <div className="max-w-xl">
            By using this software (OpenFire), you agree to the following terms
            and conditions as defined under the{" "}
            <a
              href="#"
              onClick={() =>
                open(
                  "https://github.com/fire-library/openfire/blob/main/LICENSE.txt"
                )
              }
              className="text-blue-600"
            >
              MIT License
            </a>
            . This agreement governs your use of the software and outlines your
            rights and responsibilities.
          </div>
          <h2 className="text-base font-semibold leading-6 text-gray-900">
            License Terms{" "}
          </h2>
          <ol className="list-decimal list-inside text-sm flex flex-col gap-2">
            <li>
              Permission is Granted: You are free to use, copy, modify, merge,
              publish, distribute, sublicense, and/or sell copies of this
              software, subject to the conditions below.
            </li>

            <li>
              Condition of Use: A copy of the original MIT License text, along
              with this software, must be included in all copies or substantial
              portions of the software.
            </li>

            <li>
              Disclaimer of Warranty: THE SOFTWARE IS PROVIDED "AS IS", WITHOUT
              WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT
              LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
              PARTICULAR PURPOSE, AND NON-INFRINGEMENT.
            </li>

            <li>
              Limitation of Liability: IN NO EVENT SHALL THE AUTHORS OR
              COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES, OR OTHER
              LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT, OR OTHERWISE,
              ARISING FROM, OUT OF, OR IN CONNECTION WITH THE SOFTWARE OR THE
              USE OR OTHER DEALINGS IN THE SOFTWARE.
            </li>
          </ol>
          <h2 className="text-base font-semibold leading-6 text-gray-900">
            Responsibilities of Users
          </h2>
          <ul className="list-disc list-inside text-sm flex flex-col gap-2">
            <li>
              You are solely responsible for ensuring that your use of the
              software complies with all applicable laws and regulations.
            </li>

            <li>
              If you redistribute or modify the software, you are responsible
              for ensuring that recipients also receive a copy of the original
              MIT License text.
            </li>
          </ul>
        </div>
        <div className="text-center mt-5">
          <Success onClick={agree}>I Agree</Success>
        </div>
      </Dialog>
    );

  return null;
}
