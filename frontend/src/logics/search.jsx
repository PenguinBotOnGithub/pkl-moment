const search = (value, setVisible, items) => {
  const searchTerm = value.trim().toLowerCase(); // Trim and convert search term to lowercase
  const filteredItems = items.filter((item) =>
    item.name.toLowerCase().startsWith(searchTerm)
  );
  setVisible(filteredItems);
};

export default search;
