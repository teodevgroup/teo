import Foundation

public enum SortOrder: String, Encodable {
    case asc = "asc"
    case desc = "desc"
}

public struct Response<Data>: Decodable where Data: Decodable {
    public let data: Data
}

public struct ResponseWithMeta<Meta, Data>: Decodable where Meta: Decodable, Data: Decodable {
    public let meta: Meta
    public let data: Data
}

public struct PagingInfo {
    public let count: Int
    public let numberOfPages: Int?
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

public struct Null: Encodable {
    fileprivate init() { }
    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()
        try container.encodeNil()
    }
}

public let null = Null()

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

extension NullOr: ExpressibleByBooleanLiteral where T: ExpressibleByBooleanLiteral {
    public typealias BooleanLiteralType = T.BooleanLiteralType
    public init(booleanLiteral value: T.BooleanLiteralType) {
        self = .nonnull(T(booleanLiteral: value))
    }
}

extension NullOr: ExpressibleByIntegerLiteral where T: ExpressibleByIntegerLiteral {
    public typealias IntegerLiteralType = T.IntegerLiteralType
    public init(integerLiteral value: T.IntegerLiteralType) {
        self = .nonnull(T(integerLiteral: value))
    }
}

extension NullOr: ExpressibleByFloatLiteral where T: ExpressibleByFloatLiteral {
    public typealias FloatLiteralType = T.FloatLiteralType
    public init(floatLiteral value: T.FloatLiteralType) {
        self = .nonnull(T(floatLiteral: value))
    }
}

public class NumberFilter<T: Encodable>: Encodable {
    public let equals: T?
    public let `in`: [T]?
    public let notIn: [T]?
    public let lt: T?
    public let lte: T?
    public let gt: T?
    public let gte: T?
    public let not: NumberFilter<T>?
    public init(
        equals: T? = nil,
        `in`: [T]? = nil,
        notIn: [T]? = nil,
        lt: T? = nil,
        lte: T? = nil,
        gt: T? = nil,
        gte: T? = nil,
        not: NumberFilter<T>? = nil
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
}

public typealias ObjectIdFilter = NumberFilter<String>
public typealias Int32Filter = NumberFilter<Int32>
public typealias Int64Filter = NumberFilter<Int64>
public typealias FloatFilter = NumberFilter<Float>
public typealias DoubleFilter = NumberFilter<Double>
public typealias DecimalFilter = NumberFilter<Decimal>
public typealias DateFilter = NumberFilter<String>
public typealias DateTimeFilter = NumberFilter<Date>

public class NumberNullableFilter<T: Encodable>: Encodable {
    public let equals: NullOr<T>?
    public let `in`: [T]?
    public let notIn: [T]?
    public let lt: T?
    public let lte: T?
    public let gt: T?
    public let gte: T?
    public let not: NullOr<NumberNullableFilter<T>>?
    public init(
        equals: NullOr<T>? = nil,
        `in`: [T]? = nil,
        notIn: [T]? = nil,
        lt: T? = nil,
        lte: T? = nil,
        gt: T? = nil,
        gte: T? = nil,
        not: NullOr<NumberNullableFilter<T>>? = nil
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
}

public typealias ObjectIdNullableFilter = NumberNullableFilter<String>
public typealias Int32NullableFilter = NumberNullableFilter<Int32>
public typealias Int64NullableFilter = NumberNullableFilter<Int64>
public typealias FloatNullableFilter = NumberNullableFilter<Float>
public typealias DoubleNullableFilter = NumberNullableFilter<Double>
public typealias DecimalNullableFilter = NumberNullableFilter<Decimal>
public typealias DateNullableFilter = NumberNullableFilter<String>
public typealias DateTimeNullableFilter = NumberNullableFilter<Date>


public class BoolFilter: Encodable, ExpressibleByBooleanLiteral {
    public let equals: Bool?
    public let not: BoolFilter?
    public init(
        equals: Bool? = nil,
        not: BoolFilter? = nil
    ) {
        self.equals = equals
        self.not = not
    }
    public required init(booleanLiteral value: Bool) {
        self.equals = value
        self.not = nil
    }
}

public class BoolNullableFilter: Encodable, ExpressibleByBooleanLiteral {
    public let equals: NullOr<Bool>?
    public let not: BoolNullableFilter?
    public init(
        equals: NullOr<Bool>? = nil,
        not: BoolNullableFilter? = nil
    ) {
        self.equals = equals
        self.not = not
    }
    public required init(booleanLiteral value: Bool) {
        self.equals = NullOr(booleanLiteral: value)
        self.not = nil
    }
    public static var null = BoolNullableFilter(equals: .null)
}

public class StringFilter: Encodable, ExpressibleByStringLiteral {
    public typealias StringLiteralType = String
    public let equals: String?
    public let `in`: [String]?
    public let notIn: [String]?
    public let lt: String?
    public let lte: String?
    public let gt: String?
    public let gte: String?
    public let contains: String?
    public let startsWith: String?
    public let endsWith: String?
    public let matches: String?
    public let not: StringFilter?
    public init(
        equals: String? = nil,
        `in`: [String]? = nil,
        notIn: [String]? = nil,
        lt: String? = nil,
        lte: String? = nil,
        gt: String? = nil,
        gte: String? = nil,
        contains: String? = nil,
        startsWith: String? = nil,
        endsWith: String? = nil,
        matches: String? = nil,
        not: StringFilter? = nil
    ) {
        self.equals = equals
        self.in = `in`
        self.notIn = notIn
        self.lt = lt
        self.lte = lte
        self.gt = gt
        self.gte = gte
        self.contains = contains
        self.startsWith = startsWith
        self.endsWith = endsWith
        self.matches = matches
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
        self.contains = nil
        self.startsWith = nil
        self.endsWith = nil
        self.matches = nil
        self.not = nil
    }
}

public class StringNullableFilter: Encodable, ExpressibleByStringLiteral {
    public typealias StringLiteralType = String
    public let equals: NullOr<String>?
    public let `in`: [String?]?
    public let notIn: [String?]?
    public let lt: String?
    public let lte: String?
    public let gt: String?
    public let gte: String?
    public let contains: String?
    public let startsWith: String?
    public let endsWith: String?
    public let matches: String?
    public let not: StringNullableFilter?
    public init(
        equals: NullOr<String>? = nil,
        `in`: [String?]? = nil,
        notIn: [String?]? = nil,
        lt: String? = nil,
        lte: String? = nil,
        gt: String? = nil,
        gte: String? = nil,
        contains: String? = nil,
        startsWith: String? = nil,
        endsWith: String? = nil,
        matches: String? = nil,
        not: StringNullableFilter? = nil
    ) {
        self.equals = equals
        self.in = `in`
        self.notIn = notIn
        self.lt = lt
        self.lte = lte
        self.gt = gt
        self.gte = gte
        self.contains = contains
        self.startsWith = startsWith
        self.endsWith = endsWith
        self.matches = matches
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
        self.contains = nil
        self.startsWith = nil
        self.endsWith = nil
        self.matches = nil
        self.not = nil
    }
}

public class EnumFilter<T: Encodable>: Encodable {
    public let equals: T?
    public let `in`: [T]?
    public let notIn: [T]?
    public let not: EnumFilter<T>?
    public init(
        equals: T? = nil,
        `in`: [T]? = nil,
        notIn: [T]? = nil,
        not: EnumFilter<T>? = nil
    ) {
        self.equals = equals
        self.in = `in`
        self.notIn = notIn
        self.not = not
    }
}

public class EnumNullableFilter<T: Encodable>: Encodable {
    public let equals: NullOr<T>?
    public let `in`: [T?]?
    public let notIn: [T?]?
    public let not: EnumNullableFilter<T>?
    public init(
        equals: NullOr<T>? = nil,
        `in`: [T]? = nil,
        notIn: [T]? = nil,
        not: EnumNullableFilter<T>? = nil
    ) {
        self.equals = equals
        self.in = `in`
        self.notIn = notIn
        self.not = not
    }
}

public class ArrayFilter<T: Encodable>: Encodable {
    public let equals: [T]?
    public let has: T?
    public let hasSome: [T]?
    public let hasEvery: [T]?
    public let isEmpty: Bool?
    public let length: Int?
    public init(
        equals: [T]? = nil,
        has: T? = nil,
        hasSome: [T]? = nil,
        hasEvery: [T]? = nil,
        isEmpty: Bool? = nil,
        length: Int? = nil
    ) {
        self.equals = equals
        self.has = has
        self.hasSome = hasSome
        self.hasEvery = hasEvery
        self.isEmpty = isEmpty
        self.length = length
    }
}

public class ArrayNullableFilter<T: Encodable>: Encodable {
    public let equals: NullOr<[T]>?
    public let has: T?
    public let hasSome: [T]?
    public let hasEvery: [T]?
    public let isEmpty: Bool?
    public let length: Int?
    public init(
        equals: NullOr<[T]>? = nil,
        has: T? = nil,
        hasSome: [T]? = nil,
        hasEvery: [T]? = nil,
        isEmpty: Bool? = nil,
        length: Int? = nil
    ) {
        self.equals = equals
        self.has = has
        self.hasSome = hasSome
        self.hasEvery = hasEvery
        self.isEmpty = isEmpty
        self.length = length
    }
}

public enum NumberFieldUpdateOperation<T: Encodable>: Encodable {
    case set(T)
    case increment(T)
    case decrement(T)
    case multiply(T)
    case divide(T)
    enum CodingKeys: CodingKey {
        case set
        case increment
        case decrement
        case multiply
        case divide
    }
    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: NumberFieldUpdateOperation<T>.CodingKeys.self)
        switch self {
        case .set(let a0):
            try container.encode(a0, forKey: .set)
        case .increment(let a0):
            try container.encode(a0, forKey: .increment)
        case .decrement(let a0):
            try container.encode(a0, forKey: .decrement)
        case .multiply(let a0):
            try container.encode(a0, forKey: .multiply)
        case .divide(let a0):
            try container.encode(a0, forKey: .divide)
        }
    }
}

public enum ArrayFieldUpdateOperation<T: Encodable>: Encodable {
    case set([T])
    case push(T)
    enum CodingKeys: CodingKey {
        case set
        case push
    }
    public func encode(to encoder: Encoder) throws {
        var container = encoder.container(keyedBy: ArrayFieldUpdateOperation<T>.CodingKeys.self)
        switch self {
        case .set(let a0):
            try container.encode(a0, forKey: .set)
        case .push(let a0):
            try container.encode(a0, forKey: .push)
        }
    }
}
