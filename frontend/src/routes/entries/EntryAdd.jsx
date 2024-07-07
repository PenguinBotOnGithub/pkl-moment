import React from "react";
import Navbar from "../../components/Navbar";
import CompanyDropdown from "../../components/dropdowns/CompanyDropdown";
import AdviserDropdown from "../../components/dropdowns/AdviserDropdown";
import useEntryAddController from "./EntryAddController";
import StudentEntryAddTable from "../../components/tables/entries/StudentEntryAddTable";

function EntryAdd() {
  const {
    company,
    advisers,
    students,
    handleOnSubmit,
  } = useEntryAddController();

  return (
    <div className="flex-col flex gap-2 items-center">
      {role !== "advisor" && (
        <div className="w-full max-w-screen-sm flex-row flex gap-2 items-center">
          <label className={labelStyle}>Pembimbing</label>
          <AdviserDropdown
            value={advisers}
            setSelectedValue={setSelectedAdvisers}
          />
        </div>
      )}
      <div className="w-full max-w-screen-sm relative flex-row flex gap-2 items-center">
        <label className={labelStyle}>Perusahaan</label>
        <CompanyDropdown
          value={company}
          setSelectedValue={setSelectedCompany}
        />
      </div>
      {entry !== "penarikan" && (
        <div className="w-full max-w-screen-sm flex-row flex gap-2 items-center">
          <label className={labelStyle}>Tanggal Berangkat</label>
          <input
            type="date"
            className="input w-full"
            value={startDate}
            onChange={(e) => setStartDate(e.target.value)}
          />
        </div>
      )}
      <div className="w-full max-w-screen-sm flex-row flex gap-2 items-center">
        <label className={labelStyle}>Tanggal Kembali</label>
        <div
          role="tablist"
          className="tabs-boxed p-0 bg-base-100 gap-2 flex flex-row flex-nowrap"
        >
          {entryValue.map((entry, index) => (
            <button
              key={index}
              role="tab"
              onClick={() => handleEntryClick(index)}
              className={`tab hover:bg-base-300 ease-in-out duration-150 ${
                currentEndDate === index ? "tab-active" : ""
              }`}
            >
              {entry}
            </button>
          ))}
        </div>
        <span>{endDate}</span>
      </div>
      <StudentEntryAddTable
        rows={rows}
        onAddRow={addRow}
        onDeleteRow={deleteRow}
        onSearchStudent={searchStudent}
        isMaxWidth={true}
      />
      <button
        className="btn btn-primary max-w-screen-sm w-full"
        onClick={handleOnSubmit}
      >
        Send
      </button>
    </div>
  );
}

export default EntryAdd;
