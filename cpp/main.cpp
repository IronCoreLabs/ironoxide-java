#include <cstdlib>
#include <iostream>

#include "sdk/IronSdk.hpp"
using namespace sdk;

template<class T>
T unwrap(std::variant<T, RustString> value) {
    if (std::holds_alternative<T>(value)) {
        return std::get<T>(std::move(value));
    } else {
        std::cout << "Got fatal error: " << std::get<RustString>(value).to_string_view() << "\n";
        exit(EXIT_FAILURE);
    }
}

RustSlice<const int8_t> string_to_slice(std::string str) {
    return RustSlice{reinterpret_cast<const int8_t*>(str.data()), str.size() };
}

RustSlice<const int8_t> vec_to_slice(RustVeci8 a) {
    return RustSlice{ &a[0], a.size() };
}

std::string vec_to_string(RustVeci8 a) {
    return std::string(reinterpret_cast< char const* >(&a[0]), a.size());
}

int main()
{
    // Fire up the Getting Started repo
    // (https://github.com/ironcorelabs/getting-started)
    // and hit this URL: 
    // http://localhost:3000/generateJWT/kirk
    // Then paste that into here. You'll need to compile
    // and run the app within two minutes or do it again.
    // Hacky? yes. 
    std::string_view jwt = "eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9.eyJwaWQiOjIsInNpZCI6ImljLWdldHRpbmctc3RhcnRlZCIsImtpZCI6NTM0LCJpYXQiOjE1ODA3ODg0NzIsImV4cCI6MTU4MDc4ODU5Miwic3ViIjoia2lyayJ9.HtzNMgFb7lJF2hLkIGvHK-dD3OOraymqu4f7DI76krVPa3DsVzGZaTmNcj3EGyvyk8TY_hjyWf1TInYFEgY9-w";

    DeviceContext dc(unwrap(IronSdk::generateNewDevice(jwt, "SAMPLE_PASSCODE", DeviceCreateOpts())));
    IronSdk sdk = unwrap(IronSdk::initialize(dc));

    std::string str ("encrypt me please");
    RustSlice<const int8_t> bytes2 = string_to_slice(str);

    DocumentEncryptResult res = unwrap(sdk.documentEncrypt(std::move(bytes2), DocumentEncryptOpts()));

    std::cout << "Got " << res.getEncryptedData().size() << " bytes encrypted\n";

    DocumentDecryptResult plain = unwrap(sdk.documentDecrypt(vec_to_slice(res.getEncryptedData())));

    std::cout << "Got " << plain.getDecryptedData().size() << " bytes decrypted\n";

    std::cout << "Dude, there were no errors! \nDecrypted: " << vec_to_string(plain.getDecryptedData()) << "\n";

    return EXIT_SUCCESS;
}

