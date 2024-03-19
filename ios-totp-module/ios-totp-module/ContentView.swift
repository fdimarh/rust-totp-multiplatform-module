//
//  ContentView.swift
//  ios-totp-module
//
//  Created by fajar dimar habibi on 19/03/24.
//

import SwiftUI
import TOTP

struct ContentView: View {
    let result: UnsafeMutablePointer<CChar> = TOTP.get_pin_number(6, 1, 30, "12345678901234567890")
    
    var swiftResult: String {
                String(cString: result)
            }
    
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundStyle(.tint)
            Text(swiftResult)
        }
        .padding()
    }
}

#Preview {
    ContentView()
}
