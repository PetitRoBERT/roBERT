import * as grpc from "grpc";
import { Observable } from "rxjs";
/** Namespace database. */
export namespace database {
  /** Contains all the RPC service clients. */
  export interface ClientFactory {
    /**
     * Returns the AuthorService service client.
     */
    getAuthorService(): database.AuthorService;
  }

  /** Builder for an RPC service server. */
  export interface ServerBuilder {
    /**
     * Adds a AuthorService service implementation.
     * @param impl AuthorService service implementation
     */
    addAuthorService(impl: database.AuthorService): database.ServerBuilder;
  }

  /** Constructs a new AuthorService service. */
  export interface AuthorService {
    /**
     * Calls FindOneById.
     * @param request AuthorById message or plain object
     *  * @param metadata Optional metadata
     * @returns Promise
     */
    findOneById(
      request: database.AuthorById,
      metadata?: grpc.Metadata
    ): Observable<database.Author>;
  }

  /** Properties of an AuthorById. */
  export interface AuthorById {
    /** AuthorById id */
    id?: number | null;
  }

  /** Properties of an Author. */
  export interface Author {
    /** Author id */
    id?: number | null;

    /** Author name */
    name?: string | null;
  }
}
