import React, { useEffect, useState } from "react";
import Cookies from "universal-cookie";
import { useNavigate, useParams } from "react-router-dom";
import host from "../../assets/strings/host";
import StatisticUser from "../../components/count/StatisticUser";
import Search from "../../components/Search";

function AllUsers() {
  const navigate = useNavigate();
  const [data, setData] = useState([]);
  const [pageData, setPageData] = useState();
  const { page } = useParams();
  const cookies = new Cookies();
  const token = cookies.get("access-token");

  const [selectedUser, setSelectedUser] = useState(null);
  const [newPassword, setNewPassword] = useState("");
  const [showPassword, setShowPassword] = useState(false);

  function handlePageChange(index) {
    navigate(`/admin/users/${index}`);
  }

  const fetchDataForUsers = async () => {
    try {
      const response = await fetch(
        `${host}/api/user?page=${page}&size=${cookies.get("max-item-users")}`,
        {
          headers: {
            Authorization: `Bearer ${token}`,
          },
        }
      );
      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      const usersData = await response.json();
      console.log(usersData);
      setData(usersData.data.items);
      setPageData(usersData.data);
    } catch (err) {
      alert("Something went wrong: " + err);
      setData([]);
    }
  };

  const onDelete = async (id) => {
    try {
      const response = await fetch(`${host}/api/user/${id}/delete`, {
        method: "DELETE",
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });
      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      fetchDataForUsers();
    } catch (err) {
      alert("Something went wrong: " + err);
    }
  };

  const onChangePassword = async () => {
    try {
      const response = await fetch(
        `${host}/api/user/${selectedUser}/change-password`,
        {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
            Authorization: `Bearer ${token}`,
          },
          body: JSON.stringify({ newPassword }),
        }
      );
      if (!response.ok) {
        throw new Error(`HTTP error: Status ${response.status}`);
      }
      setNewPassword("");
      setSelectedUser(null);
      fetchDataForUsers();
    } catch (err) {
      alert("Something went wrong: " + err);
    }
  };

  useEffect(() => {
    fetchDataForUsers();
  }, [page]);

  return (
    <>
      <Search addOnClick={() => navigate("/admin/users/add")} />
      <StatisticUser />
      <div className="overflow-x-auto">
        <table className="table bg-base-100 border-0 overflow-hidden rounded-lg">
          <thead className="bg-base-300">
            <tr>
              <th className="w-0">No</th>
              <th>Username</th>
              <th>Role</th>
              <th>Aksi</th>
            </tr>
          </thead>
          <tbody>
            {data.map((row, index) => (
              <tr key={row.id} className="border-t-2 border-base-300">
                <td>{index + 1}</td>
                <td>{row.username}</td>
                <td>{row.role}</td>
                <td>
                  <button
                    className="btn btn-warning btn-xs rounded-lg mr-2"
                    onClick={() => setSelectedUser(row.id)}
                  >
                    Ganti Password
                  </button>
                  {row.role !== "secretary" && (
                    <button
                      className="btn btn-error btn-xs rounded-lg"
                      onClick={() => onDelete(row.id)}
                    >
                      Delete
                    </button>
                  )}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
        {pageData && (
          <div className="flex justify-center items-center gap-2 mt-2">
            <button
              className="flex-none btn bg-base-100"
              onClick={() => handlePageChange(pageData.page - 1)}
              disabled={pageData.page === 0}
            >
              <span className="material-symbols-rounded icon-size-20">
                arrow_back
              </span>
            </button>
            <div className="join flex">
              {[...Array(pageData.num_pages)].map((_, index) => (
                <button
                  key={index}
                  className={`join-item btn ${
                    pageData.page === index
                      ? "bg-primary text-primary-content"
                      : "bg-base-100"
                  }`}
                  onClick={() => handlePageChange(index)}
                >
                  {index + 1}
                </button>
              ))}
            </div>
            <button
              className="flex-none btn bg-base-100"
              onClick={() => handlePageChange(pageData.page + 1)}
              disabled={pageData.page === pageData.num_pages - 1}
            >
              <span className="material-symbols-rounded icon-size-20">
                arrow_forward
              </span>
            </button>
          </div>
        )}
      </div>
      {selectedUser && (
        <dialog open className="modal">
          <div className="modal-box">
            <form
              onSubmit={(e) => {
                e.preventDefault();
                onChangePassword();
              }}
            >
              <button
                type="button"
                className="btn btn-sm btn-circle btn-ghost absolute right-2 top-2"
                onClick={() => setSelectedUser(null)}
              >
                <span className="material-symbols-rounded">close</span>
              </button>
              <h3 className="text-lg font-bold">Change Password</h3>
              <input
                type={showPassword ? "text" : "password"}
                placeholder="Password"
                className="input input-bordered w-full my-2"
                value={newPassword}
                onChange={(e) => setNewPassword(e.target.value)}
                required
              />
              <input
                type={showPassword ? "text" : "password"}
                placeholder="Confirm Password"
                className="input input-bordered w-full my-2"
                value={newPassword}
                onChange={(e) => setNewPassword(e.target.value)}
                required
              />
              <div className="flex items-center">
                <input
                  type="checkbox"
                  checked={showPassword}
                  onChange={() => setShowPassword(!showPassword)}
                  className="mr-2"
                />
                <label>Show Password</label>
              </div>
              <button type="submit" className="btn btn-primary w-full my-2">
                Submit
              </button>
            </form>
          </div>
        </dialog>
      )}
    </>
  );
}

export default AllUsers;
