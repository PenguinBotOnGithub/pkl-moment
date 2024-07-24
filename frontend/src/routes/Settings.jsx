import React from "react";
import { useTranslation } from "react-i18next";
import Cookies from "universal-cookie";
import ThemeController from "../components/ThemeController";
import PKLMomentIconSimple from "../assets/drawable/PKLMomentIconSimple";

function Settings({ cookies }) {
  const { t } = useTranslation();
  const username = cookies.get("user-name");

  return (
    <div className="flex flex-col items-center gap-2">
      <div className="flex gap-2 w-full max-w-xl">
        <div className="bg-base-100 p-4 rounded-btn flex flex-col items-start flex-1">
          <div className="flex items-center mb-4">
            <span className="material-symbols-rounded icon-size-24">
              person
            </span>
            <span className="ml-2 text-lg font-bold">Pengguna</span>
          </div>
          <div className="w-full">
            <div className="flex items-center rounded-box border-2 border-neutral">
              <div className="p-2 flex-1 border-r-2 border-neutral">
                Username
              </div>
              <div className="p-2 flex-1">{username}</div>
            </div>
          </div>
        </div>
      </div>

      <div className="flex justify-between gap-2 w-full max-w-xl">
        <div className="bg-base-100 p-4 rounded-btn flex flex-col items-start flex-1">
          <div className="flex items-center">
            <PKLMomentIconSimple size={24}/>
            <span className="ml-2 text-lg font-bold">Tampilan (Coming Soon)</span>
          </div>
          <div className="flex items-center justify-between mt-4 w-full">
            <span>Tema</span>
            <ThemeController cookies={cookies}/>
          </div>
          <div className="flex items-center justify-between mt-4 w-full">
            <span>Bahasa</span>
            <select
              defaultValue="ind"
              className="select bg-neutral ml-auto w-full max-w-xs text-neutral-content"
            >
              <option value="ind">Indonesia</option>
              <option value="eng">English</option>
              <option value="jap">Jepang</option>
            </select>
          </div>
        </div>
      </div>

      <div className="flex justify-between gap-2 w-full max-w-xl">
        <div className="bg-base-100 p-4 rounded-btn flex flex-col items-start flex-1">
          <div className="flex items-center">
            <span className="material-symbols-rounded icon-size-24">
              description
            </span>
            <span className="ml-2 text-lg font-bold">
              Konfigurasi Tabel (Coming Soon)
            </span>
          </div>
          <div className="flex items-center justify-between mt-4 w-full">
            <span>Max item di tabel</span>
            <label className="bg-base-200 input flex-none w-12 flex items-center gap-2">
              <input
                type="number"
                className="grow w-8 placeholder:text-neutral-content placeholder:opacity-50"
                defaultValue={cookies.get("max-item")}
                onChange={(event) => cookies.set("max-item",event.target.value)}
              />
            </label>
          </div>
          <div className="flex items-center justify-between mt-4 w-full">
            <span>Tampilan yang telah diverifikasi</span>
            <input type="checkbox" className="toggle ml-auto" />
          </div>
        </div>
      </div>

      <div className="flex justify-between gap-2 w-full max-w-xl">
        <button
          className="btn btn-lg bg-base-100 text-error p-4 rounded-btn flex flex-row items-center flex-1"
          onClick={() => document.getElementById("logout_confirmation_modal").showModal()}
        >
          <span className="material-symbols-rounded icon-size-24 text-error">
            logout
          </span>
          <span className="text-lg font-bold text-error">Logout</span>
        </button>

        <dialog id="logout_confirmation_modal" className="modal">
          <div className="modal-box">
            <h3 className="font-bold text-lg text-error">Warning!</h3>
            <p className="py-4">
              Are you sure you want to logout?
            </p>
            <div className="modal-action">
              <form method="dialog">
                {/* if there is a button in form, it will close the modal */}
                <button
                  className="btn text-error"
                  onClick={() => {
                    cookies.remove("access-token");
                    cookies.remove("selected-wave");
                    cookies.remove("user-id");
                    cookies.remove("user-name");
                    cookies.remove("role");
                    window.location.reload();
                  }}
                >
                  Yes
                </button>
                <button className="btn ml-2">Cancel</button>
              </form>
            </div>
          </div>
        </dialog>
      </div>

    </div>
  );
}

export default Settings;
