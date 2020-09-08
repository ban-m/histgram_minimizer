library("tidyverse")
setwd("~/work/histogram_minimizer/")


chibacorrect <- read_csv("./data/ChibaNormalFlat1000CORRECT.csv")
chibauncrrect <- read_csv("./data/ChibaNormalFlat1000UNCORRECT.csv")
subcorrect <-read_csv("./data/SubHillNormal450CORRECT.csv")
subuncorrect <- read_csv("./data/SubHillNormal450UNCORRECT.csv")
mockcorrect <- read_csv("./data/SubNormalNormal250CORRECT.csv")
mockuncorrect <- read_csv("./data/SubNormalNormal250UNCORRECT.csv")

chiba <- bind_rows(chibauncrrect,chibacorrect)
sub <- bind_rows(subcorrect,subuncorrect)
mock <- bind_rows(mockcorrect,mockuncorrect)

chibaresult <- read_csv("./result/chiba.csv",col_names = FALSE)%>% 
    rename(refsize=X1,min=X2,argmin=X3)
subresult <- read_csv("./result/sub.csv",col_names=FALSE)%>% 
    rename(refsize=X1,min=X2,argmin=X3)
mockresult <- read_csv("./result/mock.csv",col_names = FALSE)%>% 
    rename(refsize=X1,min=X2,argmin=X3)

generalplot <- function(g,name){
    filename <- paste0("./pdf/",name,".pdf")
    pdf(filename)
    plot(g)
    dev.off()
    filename <- paste0("./png/",name,".png")
    png(filename)
    plot(g)
    dev.off()
    0
}

plot_with_optimal <- function(refsize,df,name,opt){
    g <- df %>% ggplot(mapping = aes(x = score,fill = type)) +
        geom_histogram(position = "identity",alpha = 0.7,bins = 60) + 
        labs(title = paste0("histogram of ",name,"refsize:",refsize)) +
        geom_vline(xintercept = opt,colour = "blue")
    name <- paste0(name,refsize)
    generalplot(g,name)
}


chiba %>%  mutate(type = ifelse(type == 1,"Correct","Wrong")) %>%nest(-refsize) %>% 
    full_join(chibaresult,by="refsize") %>% 
    apply(MARGIN = 1,function(ls){plot_with_optimal(ls$refsize,ls$data,"ChibaNormalFlat1000",ls$argmin)})

sub %>%  mutate(type = ifelse(type == 1,"Correct","Wrong")) %>% nest(-refsize) %>%
    full_join(subresult,by="refsize") %>% 
    apply(MARGIN = 1,function(ls){plot_with_optimal(ls$refsize,ls$data,"SubHillNormal450",ls$argmin)})
mock %>% mutate(type = ifelse(type == 1,"Correct","Wrong")) %>% nest(-refsize) %>%
    full_join(mockresult,by="refsize") %>% 
    apply(MARGIN = 1,function(ls){plot_with_optimal(ls$refsize,ls$data,"SubNormalNormal250",ls$argmin)})
