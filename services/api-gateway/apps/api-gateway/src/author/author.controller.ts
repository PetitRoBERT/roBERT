import { Controller, Get, Logger, Query, Param } from '@nestjs/common';
import { AuthorService } from './author.service';
import { Observable } from 'rxjs';
import { IAuthor } from '@app/ts-interfaces';

@Controller('author')
export class AuthorController {
  private logger = new Logger('AuthorController');

  constructor(private readonly authorService: AuthorService) { }

  @Get()
  getAuthor(): Observable<IAuthor> {
    this.logger.log('Getting an author from API Gateway');
    return this.authorService.getAuthor();
  }


  @Get(':id')
  getAuthorById(@Param('id') id: number): Observable<IAuthor> {
    this.logger.log(`Getting author with id ${id} from API Gateway`);
    return this.authorService.getAuthorById(id);
  }
}
