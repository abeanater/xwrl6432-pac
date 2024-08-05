#[doc = "Register `PMPPRXY` reader"]
pub type R = crate::R<PmpprxySpec>;
#[doc = "Register `PMPPRXY` writer"]
pub type W = crate::W<PmpprxySpec>;
#[doc = "Field `PRIVILEGE_ID` reader - 3:0\\]
Privilege ID:#br#PMPPRXY.PRIVID is always updated with the value from configuration bus privilege ID field on any/every write to Program Set BIDX Register \\[trigger register\\].#br#The PRIVID value for the SA Set and DF Set are copied from the value in the Program set along with the remainder of the parameter values.The privilege ID is issued on the VBusM read and write command bus such that the target endpoints can perform memory protection checks based on the privid of the external host that sets up the DMA transaction."]
pub type PrivilegeIdR = crate::FieldReader;
#[doc = "Field `PRIVILEGE_ID` writer - 3:0\\]
Privilege ID:#br#PMPPRXY.PRIVID is always updated with the value from configuration bus privilege ID field on any/every write to Program Set BIDX Register \\[trigger register\\].#br#The PRIVID value for the SA Set and DF Set are copied from the value in the Program set along with the remainder of the parameter values.The privilege ID is issued on the VBusM read and write command bus such that the target endpoints can perform memory protection checks based on the privid of the external host that sets up the DMA transaction."]
pub type PrivilegeIdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PRIVILEGE_LEVEL` reader - 8:8\\]
Privilege Level:#br#PRIV = 0 : User level privilege#br#PRIV = 1 : Supervisor level privilege#br#PMPPRXY.PRIV is always updated with the value from the configuration bus privilege field on any/every write to Program Set BIDX Register \\[trigger register\\].#br#The PRIV value for the SA Set and DF Set are copied from the value in the Program set along with the remainder of the parameter values.The privilege ID is issued on the VBusM read and write command bus such that the target endpoints can perform memory protection checks based on the PRIV of the external host that sets up the DMA transaction."]
pub type PrivilegeLevelR = crate::BitReader;
#[doc = "Field `PRIVILEGE_LEVEL` writer - 8:8\\]
Privilege Level:#br#PRIV = 0 : User level privilege#br#PRIV = 1 : Supervisor level privilege#br#PMPPRXY.PRIV is always updated with the value from the configuration bus privilege field on any/every write to Program Set BIDX Register \\[trigger register\\].#br#The PRIV value for the SA Set and DF Set are copied from the value in the Program set along with the remainder of the parameter values.The privilege ID is issued on the VBusM read and write command bus such that the target endpoints can perform memory protection checks based on the PRIV of the external host that sets up the DMA transaction."]
pub type PrivilegeLevelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SECURE_LEVEL` reader - 9:9\\]
Secure Level: Deprecated, always read as 0."]
pub type SecureLevelR = crate::BitReader;
#[doc = "Field `SECURE_LEVEL` writer - 9:9\\]
Secure Level: Deprecated, always read as 0."]
pub type SecureLevelW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Privilege ID:#br#PMPPRXY.PRIVID is always updated with the value from configuration bus privilege ID field on any/every write to Program Set BIDX Register \\[trigger register\\].#br#The PRIVID value for the SA Set and DF Set are copied from the value in the Program set along with the remainder of the parameter values.The privilege ID is issued on the VBusM read and write command bus such that the target endpoints can perform memory protection checks based on the privid of the external host that sets up the DMA transaction."]
    #[inline(always)]
    pub fn privilege_id(&self) -> PrivilegeIdR {
        PrivilegeIdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - 8:8\\]
Privilege Level:#br#PRIV = 0 : User level privilege#br#PRIV = 1 : Supervisor level privilege#br#PMPPRXY.PRIV is always updated with the value from the configuration bus privilege field on any/every write to Program Set BIDX Register \\[trigger register\\].#br#The PRIV value for the SA Set and DF Set are copied from the value in the Program set along with the remainder of the parameter values.The privilege ID is issued on the VBusM read and write command bus such that the target endpoints can perform memory protection checks based on the PRIV of the external host that sets up the DMA transaction."]
    #[inline(always)]
    pub fn privilege_level(&self) -> PrivilegeLevelR {
        PrivilegeLevelR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 9:9\\]
Secure Level: Deprecated, always read as 0."]
    #[inline(always)]
    pub fn secure_level(&self) -> SecureLevelR {
        SecureLevelR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Privilege ID:#br#PMPPRXY.PRIVID is always updated with the value from configuration bus privilege ID field on any/every write to Program Set BIDX Register \\[trigger register\\].#br#The PRIVID value for the SA Set and DF Set are copied from the value in the Program set along with the remainder of the parameter values.The privilege ID is issued on the VBusM read and write command bus such that the target endpoints can perform memory protection checks based on the privid of the external host that sets up the DMA transaction."]
    #[inline(always)]
    #[must_use]
    pub fn privilege_id(&mut self) -> PrivilegeIdW<PmpprxySpec> {
        PrivilegeIdW::new(self, 0)
    }
    #[doc = "Bit 8 - 8:8\\]
Privilege Level:#br#PRIV = 0 : User level privilege#br#PRIV = 1 : Supervisor level privilege#br#PMPPRXY.PRIV is always updated with the value from the configuration bus privilege field on any/every write to Program Set BIDX Register \\[trigger register\\].#br#The PRIV value for the SA Set and DF Set are copied from the value in the Program set along with the remainder of the parameter values.The privilege ID is issued on the VBusM read and write command bus such that the target endpoints can perform memory protection checks based on the PRIV of the external host that sets up the DMA transaction."]
    #[inline(always)]
    #[must_use]
    pub fn privilege_level(&mut self) -> PrivilegeLevelW<PmpprxySpec> {
        PrivilegeLevelW::new(self, 8)
    }
    #[doc = "Bit 9 - 9:9\\]
Secure Level: Deprecated, always read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn secure_level(&mut self) -> SecureLevelW<PmpprxySpec> {
        SecureLevelW::new(self, 9)
    }
}
#[doc = "Prog Set Mem Protect Proxy\n\nYou can [`read`](crate::Reg::read) this register and get [`pmpprxy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmpprxy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmpprxySpec;
impl crate::RegisterSpec for PmpprxySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmpprxy::R`](R) reader structure"]
impl crate::Readable for PmpprxySpec {}
#[doc = "`write(|w| ..)` method takes [`pmpprxy::W`](W) writer structure"]
impl crate::Writable for PmpprxySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMPPRXY to value 0"]
impl crate::Resettable for PmpprxySpec {
    const RESET_VALUE: u32 = 0;
}
