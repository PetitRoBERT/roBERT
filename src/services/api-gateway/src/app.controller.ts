import { Controller, Get, Logger } from '@nestjs/common';
import { database } from '../../../interfaces/database/api';
import { AppService } from './app.service';
import { Observable } from 'rxjs';

@Controller()
export class AppController {
  private logger = new Logger('AppController');

  constructor(private readonly appService: AppService) {}

  @Get()
  getAuthor(): Observable<database.Author> {
    this.logger.log('Getting an author from API Gateway');
    return this.appService.getAuthor();
  }
}
