#[doc = "Register `fault_attr_status` reader"]
pub type R = crate::R<FaultAttrStatusSpec>;
#[doc = "Register `fault_attr_status` writer"]
pub type W = crate::W<FaultAttrStatusSpec>;
#[doc = "Field `PRIVILEGE_ID_` reader - 7:0\\]
Privilege ID."]
pub type PrivilegeId_R = crate::FieldReader;
#[doc = "Field `PRIVILEGE_ID_` writer - 7:0\\]
Privilege ID."]
pub type PrivilegeId_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ROUTE_ID_` reader - 19:8\\]
Route ID."]
pub type RouteId_R = crate::FieldReader<u16>;
#[doc = "Field `ROUTE_ID_` writer - 19:8\\]
Route ID."]
pub type RouteId_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `XID_` reader - 31:20\\]
XID."]
pub type Xid_R = crate::FieldReader<u16>;
#[doc = "Field `XID_` writer - 31:20\\]
XID."]
pub type Xid_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Privilege ID."]
    #[inline(always)]
    pub fn privilege_id_(&self) -> PrivilegeId_R {
        PrivilegeId_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:19 - 19:8\\]
Route ID."]
    #[inline(always)]
    pub fn route_id_(&self) -> RouteId_R {
        RouteId_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - 31:20\\]
XID."]
    #[inline(always)]
    pub fn xid_(&self) -> Xid_R {
        Xid_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Privilege ID."]
    #[inline(always)]
    #[must_use]
    pub fn privilege_id_(&mut self) -> PrivilegeId_W<FaultAttrStatusSpec> {
        PrivilegeId_W::new(self, 0)
    }
    #[doc = "Bits 8:19 - 19:8\\]
Route ID."]
    #[inline(always)]
    #[must_use]
    pub fn route_id_(&mut self) -> RouteId_W<FaultAttrStatusSpec> {
        RouteId_W::new(self, 8)
    }
    #[doc = "Bits 20:31 - 31:20\\]
XID."]
    #[inline(always)]
    #[must_use]
    pub fn xid_(&mut self) -> Xid_W<FaultAttrStatusSpec> {
        Xid_W::new(self, 20)
    }
}
#[doc = "Fault Attribute Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`fault_attr_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fault_attr_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FaultAttrStatusSpec;
impl crate::RegisterSpec for FaultAttrStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fault_attr_status::R`](R) reader structure"]
impl crate::Readable for FaultAttrStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`fault_attr_status::W`](W) writer structure"]
impl crate::Writable for FaultAttrStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets fault_attr_status to value 0"]
impl crate::Resettable for FaultAttrStatusSpec {
    const RESET_VALUE: u32 = 0;
}
