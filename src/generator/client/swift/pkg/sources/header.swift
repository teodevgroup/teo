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

struct AnyEncodable: Encodable {
    let value: any Encodable
    func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()
        try! container.encode(value)
    }
}

public enum NullOr<T>: Encodable where T: Encodable {
    case null
    case nonnull(T)
    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()
        switch self {
        case .null:
            try! container.encodeNil()
        case .nonnull(let t):
            try! container.encode(t)
        }
    }
}

extension NullOr: ExpressibleByUnicodeScalarLiteral where T: ExpressibleByUnicodeScalarLiteral {
    public typealias UnicodeScalarLiteralType = T.UnicodeScalarLiteralType
    public init(unicodeScalarLiteral value: T.UnicodeScalarLiteralType) {
        self = .nonnull(T(unicodeScalarLiteral: value))
    }
}

extension NullOr: ExpressibleByExtendedGraphemeClusterLiteral where T: ExpressibleByExtendedGraphemeClusterLiteral {
    public typealias ExtendedGraphemeClusterLiteralType = T.ExtendedGraphemeClusterLiteralType
    public init(extendedGraphemeClusterLiteral value: T.ExtendedGraphemeClusterLiteralType) {
        self = .nonnull(T(extendedGraphemeClusterLiteral: value))
    }
}

extension NullOr: ExpressibleByStringLiteral where T: ExpressibleByStringLiteral {
    public typealias StringLiteralType = T.StringLiteralType
    public init(stringLiteral value: T.StringLiteralType) {
        self = .nonnull(T(stringLiteral: value))
    }
}

public class ObjectIdFilter: Encodable, ExpressibleByStringLiteral {
    public typealias StringLiteralType = String
    public let equals: String?
    public let `in`: [String]?
    public let notIn: [String]?
    public let lt: String?
    public let lte: String?
    public let gt: String?
    public let gte: String?
    public let not: ObjectIdFilter?
    public init(
        equals: String? = nil,
        `in`: [String]? = nil,
        notIn: [String]? = nil,
        lt: String? = nil,
        lte: String? = nil,
        gt: String? = nil,
        gte: String? = nil,
        not: ObjectIdFilter? = nil
    ) {
        self.equals = equals
        self.in = `in`
        self.notIn = notIn
        self.lt = lt
        self.lte = lte
        self.gt = gt
        self.gte = gte
        self.not = not
    }
    public required init(stringLiteral value: String) {
        self.equals = value
        self.in = nil
        self.notIn = nil
        self.lt = nil
        self.lte = nil
        self.gt = nil
        self.gte = nil
        self.not = nil
    }
}

public class ObjectIdNullableFilter: Encodable, ExpressibleByStringLiteral {
    public typealias StringLiteralType = String
    public var equals: NullOr<String>?
    public let `in`: [String?]?
    public let notIn: [String?]?
    public let lt: String?
    public let lte: String?
    public let gt: String?
    public let gte: String?
    public let not: ObjectIdNullableFilter?
    public init(
        equals: NullOr<String>? = nil,
        `in`: [String?]? = nil,
        notIn: [String?]? = nil,
        lt: String? = nil,
        lte: String? = nil,
        gt: String? = nil,
        gte: String? = nil,
        not: ObjectIdNullableFilter? = nil
    ) {
        self.equals = equals
        self.in = `in`
        self.notIn = notIn
        self.lt = lt
        self.lte = lte
        self.gt = gt
        self.gte = gte
        self.not = not
    }
    public required init(stringLiteral value: String) {
        self.equals = .nonnull(value)
        self.in = nil
        self.notIn = nil
        self.lt = nil
        self.lte = nil
        self.gt = nil
        self.gte = nil
        self.not = nil
    }
    public static var null = ObjectIdNullableFilter(equals: .null)
}
