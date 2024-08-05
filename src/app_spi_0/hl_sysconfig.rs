#[doc = "Register `HL_SYSCONFIG` reader"]
pub type R = crate::R<HlSysconfigSpec>;
#[doc = "Register `HL_SYSCONFIG` writer"]
pub type W = crate::W<HlSysconfigSpec>;
#[doc = "Field `SOFTRESET` reader - 0:0\\]
Software reset \\[Optional\\]
- (RW )"]
pub type SoftresetR = crate::BitReader;
#[doc = "Field `SOFTRESET` writer - 0:0\\]
Software reset \\[Optional\\]
- (RW )"]
pub type SoftresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FREEEMU` reader - 1:1\\]
Sensitivity to emulation \\[debug\\]
suspend input signal - (RW )"]
pub type FreeemuR = crate::BitReader;
#[doc = "Field `FREEEMU` writer - 1:1\\]
Sensitivity to emulation \\[debug\\]
suspend input signal - (RW )"]
pub type FreeemuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDLEMODE` reader - 3:2\\]
Configuration of the local target state management mode By definition target can handle read/write transaction as long as it is out of IDLE state - (RW )"]
pub type IdlemodeR = crate::FieldReader;
#[doc = "Field `IDLEMODE` writer - 3:2\\]
Configuration of the local target state management mode By definition target can handle read/write transaction as long as it is out of IDLE state - (RW )"]
pub type IdlemodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RSVD` reader - 31:4\\]
Reserved - (RO )"]
pub type RsvdR = crate::FieldReader<u32>;
#[doc = "Field `RSVD` writer - 31:4\\]
Reserved - (RO )"]
pub type RsvdW<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Software reset \\[Optional\\]
- (RW )"]
    #[inline(always)]
    pub fn softreset(&self) -> SoftresetR {
        SoftresetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Sensitivity to emulation \\[debug\\]
suspend input signal - (RW )"]
    #[inline(always)]
    pub fn freeemu(&self) -> FreeemuR {
        FreeemuR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Configuration of the local target state management mode By definition target can handle read/write transaction as long as it is out of IDLE state - (RW )"]
    #[inline(always)]
    pub fn idlemode(&self) -> IdlemodeR {
        IdlemodeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Reserved - (RO )"]
    #[inline(always)]
    pub fn rsvd(&self) -> RsvdR {
        RsvdR::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Software reset \\[Optional\\]
- (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn softreset(&mut self) -> SoftresetW<HlSysconfigSpec> {
        SoftresetW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Sensitivity to emulation \\[debug\\]
suspend input signal - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn freeemu(&mut self) -> FreeemuW<HlSysconfigSpec> {
        FreeemuW::new(self, 1)
    }
    #[doc = "Bits 2:3 - 3:2\\]
Configuration of the local target state management mode By definition target can handle read/write transaction as long as it is out of IDLE state - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn idlemode(&mut self) -> IdlemodeW<HlSysconfigSpec> {
        IdlemodeW::new(self, 2)
    }
    #[doc = "Bits 4:31 - 31:4\\]
Reserved - (RO )"]
    #[inline(always)]
    #[must_use]
    pub fn rsvd(&mut self) -> RsvdW<HlSysconfigSpec> {
        RsvdW::new(self, 4)
    }
}
#[doc = "Clock management configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hl_sysconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hl_sysconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HlSysconfigSpec;
impl crate::RegisterSpec for HlSysconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hl_sysconfig::R`](R) reader structure"]
impl crate::Readable for HlSysconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`hl_sysconfig::W`](W) writer structure"]
impl crate::Writable for HlSysconfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HL_SYSCONFIG to value 0"]
impl crate::Resettable for HlSysconfigSpec {
    const RESET_VALUE: u32 = 0;
}
