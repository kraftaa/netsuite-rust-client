# NetSuite REST API Endpoints

## **Important Note**

The exact endpoint structure can vary depending on:
- **NetSuite version** (2023.1, 2023.2, 2024.1, etc.)
- **Account type** (Production, Sandbox, BETA)
- **API version** (v1, v2, etc.)

## **Current Endpoints Used in This Client**

### **Base URL**
```
https://rest.na1.netsuite.com          # Production
https://rest.sandbox.netsuite.com      # Sandbox
```

### **Record Endpoints**
```
/rest/platform/v1/record/customer      # Customer records
/rest/platform/v1/record/vendor        # Vendor records
/rest/platform/v1/record/check         # Check records (vendor payments)
/rest/platform/v1/record/salesorder    # Sales order records
/rest/platform/v1/record/invoice       # Invoice records
/rest/platform/v1/record/bill          # Bill records
/rest/platform/v1/record/vendorbill    # Vendor bill records
/rest/platform/v1/record/cashsale      # Cash sale records
/rest/platform/v1/record/deposit       # Deposit records
/rest/platform/v1/record/inventoryitem # Inventory item records
/rest/platform/v1/record/employee      # Employee records
```

## **Key Differences from Generic SQL**

### **1. No Generic "Transaction" Endpoint**
NetSuite doesn't have a generic `/record/transaction` endpoint. You must specify the specific record type:
- ✅ `/record/check` for vendor payments
- ✅ `/record/salesorder` for sales orders
- ✅ `/record/invoice` for invoices

### **2. Query Language is NSQL, not SQL**
```sql
-- SQL (NOT supported)
SELECT * FROM Transaction WHERE Type='VendPymt'

-- NSQL (NetSuite Query Language - supported)
type IS VendPymt
createddate BETWEEN '2024-05-01' AND '2024-08-31'
```

### **3. Date Format**
```
YYYY-MM-DD  # ISO format (supported)
MM/DD/YYYY  # US format (may not work)
```

## 🔧 **How to Verify Correct Endpoints**

### **1. Check NetSuite Documentation**
- Go to NetSuite Help: REST API > REST API Reference
- Look for your specific NetSuite version
- Check the "Record Types" section

### **2. Test with Postman/Insomnia**
- Use your real NetSuite credentials
- Test different endpoints
- Check response codes and error messages

### **3. Check NetSuite Account Settings**
- Go to Setup > Integration > REST API
- Verify which API version is enabled
- Check if specific record types are accessible

## 📝 **Example Usage**

### **Vendor Payments (Checks)**
```rust
// Correct endpoint: /rest/platform/v1/record/check
let vendor_payments = client.get_vendor_payments("2024-01-01", "2024-03-31", Some(50)).await?;
```

### **Sales Orders**
```rust
// Correct endpoint: /rest/platform/v1/record/salesorder
let sales_orders = client.get_sales_orders(&["createddate BETWEEN '2024-01-01' AND '2024-12-31'"], Some(100)).await?;
```

### **Customers**
```rust
// Correct endpoint: /rest/platform/v1/record/customer
let customers = client.get_customers(Some(25)).await?;
```

## 🚀 **Adding New Record Types**

To add support for a new record type:

1. **Add the method** to `NetSuiteClient`
2. **Use the correct endpoint** from the list above
3. **Create appropriate data structures** for the response
4. **Add CLI commands** for testing

## 💡 **Pro Tips**

1. **Start with known working endpoints** like `/record/customer`
2. **Test in sandbox first** before production
3. **Check NetSuite release notes** for API changes
4. **Use proper error handling** - endpoints can change between versions
5. **Keep endpoint URLs configurable** for easy updates

## 🔍 **Troubleshooting**

### **Common Issues**
- **404 Not Found**: Endpoint doesn't exist or is incorrect
- **403 Forbidden**: Record type not accessible or permissions issue
- **400 Bad Request**: Query syntax error or invalid parameters

### **Debug Steps**
1. Verify the endpoint URL is correct
2. Check if the record type is enabled in your NetSuite account
3. Verify your OAuth permissions include the record type
4. Test with a simple query first (no filters)
5. Check NetSuite system status for API issues

---

**Remember**: Always verify endpoints against your specific NetSuite version and account configuration!
