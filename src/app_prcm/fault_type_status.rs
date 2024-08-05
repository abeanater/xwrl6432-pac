#[doc = "Register `fault_type_status` reader"]
pub type R = crate::R<FaultTypeStatusSpec>;
#[doc = "Register `fault_type_status` writer"]
pub type W = crate::W<FaultTypeStatusSpec>;
#[doc = "Field `FAULT_TYPE_` reader - 5:0\\]
Fault Type #br# 10_0000 = Supervisor read fault - priv = 1 dir = 1 dtype != 1 #br# 01_0000 = Supervisor write fault - priv = 1 dir = 0 #br# 00_1000 = Supervisor execute fault - priv = 1 dir = 1 dtype = 1 #br# 00_0100 = User read fault - priv = 0 dir = 1 dtype = 1 #br# 00_0010 = User write fault - priv = 0 dir = 0 #br# 00_0001 = User execute fault - priv = 0 dir = 1 dtype = 1 #br# 00_0000 = No fault"]
pub type FaultType_R = crate::FieldReader;
#[doc = "Field `FAULT_TYPE_` writer - 5:0\\]
Fault Type #br# 10_0000 = Supervisor read fault - priv = 1 dir = 1 dtype != 1 #br# 01_0000 = Supervisor write fault - priv = 1 dir = 0 #br# 00_1000 = Supervisor execute fault - priv = 1 dir = 1 dtype = 1 #br# 00_0100 = User read fault - priv = 0 dir = 1 dtype = 1 #br# 00_0010 = User write fault - priv = 0 dir = 0 #br# 00_0001 = User execute fault - priv = 0 dir = 1 dtype = 1 #br# 00_0000 = No fault"]
pub type FaultType_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NONSECURE_ACCESS_` reader - 6:6\\]
Non-secure access."]
pub type NonsecureAccess_R = crate::BitReader;
#[doc = "Field `NONSECURE_ACCESS_` writer - 6:6\\]
Non-secure access."]
pub type NonsecureAccess_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Fault Type #br# 10_0000 = Supervisor read fault - priv = 1 dir = 1 dtype != 1 #br# 01_0000 = Supervisor write fault - priv = 1 dir = 0 #br# 00_1000 = Supervisor execute fault - priv = 1 dir = 1 dtype = 1 #br# 00_0100 = User read fault - priv = 0 dir = 1 dtype = 1 #br# 00_0010 = User write fault - priv = 0 dir = 0 #br# 00_0001 = User execute fault - priv = 0 dir = 1 dtype = 1 #br# 00_0000 = No fault"]
    #[inline(always)]
    pub fn fault_type_(&self) -> FaultType_R {
        FaultType_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
Non-secure access."]
    #[inline(always)]
    pub fn nonsecure_access_(&self) -> NonsecureAccess_R {
        NonsecureAccess_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Fault Type #br# 10_0000 = Supervisor read fault - priv = 1 dir = 1 dtype != 1 #br# 01_0000 = Supervisor write fault - priv = 1 dir = 0 #br# 00_1000 = Supervisor execute fault - priv = 1 dir = 1 dtype = 1 #br# 00_0100 = User read fault - priv = 0 dir = 1 dtype = 1 #br# 00_0010 = User write fault - priv = 0 dir = 0 #br# 00_0001 = User execute fault - priv = 0 dir = 1 dtype = 1 #br# 00_0000 = No fault"]
    #[inline(always)]
    #[must_use]
    pub fn fault_type_(&mut self) -> FaultType_W<FaultTypeStatusSpec> {
        FaultType_W::new(self, 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Non-secure access."]
    #[inline(always)]
    #[must_use]
    pub fn nonsecure_access_(&mut self) -> NonsecureAccess_W<FaultTypeStatusSpec> {
        NonsecureAccess_W::new(self, 6)
    }
}
#[doc = "Fault Type Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fault_type_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fault_type_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FaultTypeStatusSpec;
impl crate::RegisterSpec for FaultTypeStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fault_type_status::R`](R) reader structure"]
impl crate::Readable for FaultTypeStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`fault_type_status::W`](W) writer structure"]
impl crate::Writable for FaultTypeStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets fault_type_status to value 0"]
impl crate::Resettable for FaultTypeStatusSpec {
    const RESET_VALUE: u32 = 0;
}
