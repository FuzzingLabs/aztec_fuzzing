class Variables :
    def __init__(self, name, visibility, type_, value):
        self.name = name
        self.visibility= visibility
        self.type_ = type_
        self.value = value
    
    def isPublic(self):
        return self.visibility
    
    def getName(self):
        return self.name
    
    def getType(self):
        return self.type_
    
    def getValue(self):
        return self.value