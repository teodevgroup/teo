import Foundation

public enum SortOrder: String, Encodable {
    case asc = "asc"
    case desc = "desc"
}

public struct Response<Meta, Data> where Meta: Decodable, Data: Decodable {
    public let meta: Meta
    public let data: Data
}

public struct PagingInfo {
    public let count: Int
    public let nunmberOfPages: Int?
}

public struct TokenInfo {
    public let token: String
}

public struct TeoError: Error, LocalizedError, Decodable {
    public let type: String
    public let message: String
    public let errors: Dictionary<String, String>?
}

private let tokenKey: String = "__teo_bearer_token"

private func setBearerToken(token: String) {
    UserDefaults.standard.set(token, forKey: tokenKey)
}

private func getBearerToken() -> String? {
    UserDefaults.standard.string(forKey: tokenKey)
}

private func request<I: Encodable, O: Decodable>(model: String, action: String, input: I, token: String? = getBearerToken()) async -> O {
    let url = URL(string: "http://127.0.0.1:5300/" + model + "/action/" + action)!
    var request = URLRequest(url: url)
    if let token {
        request.setValue("Bearer \(token)", forHTTPHeaderField: "Authorization")
    }
    request.httpMethod = "POST"
    request.httpBody = try! JSONEncoder().encode(input)
    let (data, response) = try! await URLSession.shared.data(for: request)
    guard response is HTTPURLResponse else { fatalError("response format is unexpected") }
    return try! JSONDecoder().decode(O.self, from: data)
}
